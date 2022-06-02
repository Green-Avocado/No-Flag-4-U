use crate::utils;
use libc::{c_char, c_int, c_void, size_t, FILE};
use std::{
    ffi::{CStr, VaList},
    mem, panic,
};

#[derive(Debug, PartialEq)]
enum FormatStringResult {
    LowRisk,
    NonConstant,
}

extern "C" {
    static stdout: *mut FILE;
}

const PERCENT_S: *const c_char = "%s\0".as_ptr() as *const c_char;

/*
    Hooks vfprintf
    - if the format string is non-constant, replace with a safe version
    - calls vfprintf in glibc with potentially modified arguments to mitigate security risks
*/
#[no_mangle]
pub unsafe extern "C" fn vfprintf(stream: *mut FILE, format: *const c_char, ap: VaList) -> c_int {
    match check_format_string(format) {
        FormatStringResult::LowRisk => {
            let real_vfprintf: extern "C" fn(*mut FILE, *const c_char, VaList) -> c_int =
                mem::transmute(utils::dlsym_next("vfprintf"));
            real_vfprintf(stream, format, ap)
        }
        FormatStringResult::NonConstant => {
            let real_fprintf: extern "C" fn(*mut FILE, *const c_char, ...) -> c_int =
                mem::transmute(utils::dlsym_next("fprintf"));
            real_fprintf(stream, PERCENT_S, format)
        }
    }
}

/*
    Hooks vprintf
    - passes call to vfprintf
*/
#[no_mangle]
pub unsafe extern "C" fn vprintf(format: *const c_char, ap: VaList) -> c_int {
    vfprintf(stdout, format, ap)
}

/*
    Hooks printf
    - passes call to vfprintf
*/
#[no_mangle]
pub unsafe extern "C" fn fprintf(stream: *mut FILE, format: *const c_char, mut args: ...) -> c_int {
    vfprintf(stream, format, args.as_va_list())
}

/*
    Hooks printf
    - passes call to vprintf
*/
#[no_mangle]
pub unsafe extern "C" fn printf(format: *const c_char, mut args: ...) -> c_int {
    vprintf(format, args.as_va_list())
}

/*
    Hooks vdprintf
    - if the format string is non-constant, replace with a safe version
    - calls vdprintf in glibc with potentially modified arguments to mitigate security risks
*/
#[no_mangle]
pub unsafe extern "C" fn vdprintf(fd: c_int, format: *const c_char, ap: VaList) -> c_int {
    match check_format_string(format) {
        FormatStringResult::LowRisk => {
            let real_vdprintf: extern "C" fn(c_int, *const c_char, VaList) -> c_int =
                mem::transmute(utils::dlsym_next("vdprintf"));
            real_vdprintf(fd, format, ap)
        }
        FormatStringResult::NonConstant => {
            let real_dprintf: extern "C" fn(c_int, *const c_char, ...) -> c_int =
                mem::transmute(utils::dlsym_next("dprintf"));
            real_dprintf(fd, PERCENT_S, format)
        }
    }
}

/*
    Hooks dprintf
    - passes call to vdprintf
*/
#[no_mangle]
pub unsafe extern "C" fn dprintf(fd: c_int, format: *const c_char, mut args: ...) -> c_int {
    vdprintf(fd, format, args.as_va_list())
}

/*
    Hooks vsprintf
    - if the format string is non-constant, replace with a safe version
    - calls vsprintf in glibc with potentially modified arguments to mitigate security risks
*/
#[no_mangle]
pub unsafe extern "C" fn vsnprintf(
    s: *mut c_char,
    size: size_t,
    format: *const c_char,
    ap: VaList,
) -> c_int {
    match check_format_string(format) {
        FormatStringResult::LowRisk => {
            let real_vsnprintf: extern "C" fn(*mut c_char, size_t, *const c_char, VaList) -> c_int =
                mem::transmute(utils::dlsym_next("vsnprintf"));
            real_vsnprintf(s, size, format, ap)
        }
        FormatStringResult::NonConstant => {
            let real_snprintf: extern "C" fn(*mut c_char, size_t, *const c_char, ...) -> c_int =
                mem::transmute(utils::dlsym_next("snprintf"));
            real_snprintf(s, size, PERCENT_S, format)
        }
    }
}

/*
    Hooks snprintf
    - passes call to vsnprintf
*/
#[no_mangle]
pub unsafe extern "C" fn snprintf(
    s: *mut c_char,
    size: size_t,
    format: *const c_char,
    mut args: ...
) -> c_int {
    vsnprintf(s, size, format, args.as_va_list())
}

/*
    Hooks vsprintf
    - if the format string is non-constant, replace with a safe version
    - calls vsprintf in glibc potentially with modified arguments to mitigate security risks
*/
#[no_mangle]
pub unsafe extern "C" fn vsprintf(s: *mut c_char, format: *const c_char, ap: VaList) -> c_int {
    match check_format_string(format) {
        FormatStringResult::LowRisk => {
            let real_vsprintf: extern "C" fn(*mut c_char, *const c_char, VaList) -> c_int =
                mem::transmute(utils::dlsym_next("vsprintf"));
            real_vsprintf(s, format, ap)
        }
        FormatStringResult::NonConstant => {
            let real_sprintf: extern "C" fn(*mut c_char, *const c_char, ...) -> c_int =
                mem::transmute(utils::dlsym_next("sprintf"));
            real_sprintf(s, PERCENT_S)
        }
    }
}

/*
    Hooks sprintf
    - passes call to vsprintf
*/
#[no_mangle]
pub unsafe extern "C" fn sprintf(s: *mut c_char, format: *const c_char, mut args: ...) -> c_int {
    vsprintf(s, format, args.as_va_list())
}

/*
    Performs sanity checks on the format string
    - returns FormatStringResult::LowRisk if everything passes
    - returns FormatStringResult::NonConstant if non-constant
    - panics if format string is dangerous
*/
fn check_format_string(format: *const c_char) -> FormatStringResult {
    let page_info =
        utils::get_ptr_info(format as *const c_void).expect("invalid format string pointer");

    if page_info.execute || !page_info.read {
        panic!("invalid format string permissions");
    }

    if page_info.file == Some("[stack]".to_string())
        || page_info.file == Some("[heap]".to_string())
        || page_info.write
    {
        return FormatStringResult::NonConstant;
    }

    if cfg!(disallow_dangerous_printf) {
        let s = unsafe { CStr::from_ptr(format) }
            .to_str()
            .expect("invalid format string");

        let mut directive_in_progress = false;

        for c in s.chars() {
            if directive_in_progress {
                match c {
                    'n' => panic!("dangerous conversion specifier prohibited"),
                    '*' | '.' | '$' => (), // field width, precision, '$'
                    '#' | '0' | '-' | ' ' | '+' | '\'' | 'I' => (), // flags
                    'h' | 'l' | 'q' | 'L' | 'j' | 'z' | 'Z' | 't' => (), // length modifier
                    d if d.is_digit(10) => (),
                    _ => directive_in_progress = false,
                }
            } else {
                if c == '%' {
                    directive_in_progress = true;
                }
            }
        }
    }

    FormatStringResult::LowRisk
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    #[test]
    #[should_panic]
    fn test_check_format_string_null() {
        _ = panic::take_hook();
        check_format_string(0 as *const c_char);
    }

    #[test]
    fn test_normal() {
        assert_eq!(
            check_format_string("%d %s Test String!\0".as_ptr() as *const c_char),
            FormatStringResult::LowRisk
        );
    }

    #[test]
    fn test_nonconstant() {
        assert_eq!(
            check_format_string(
                CString::new("%d %s Test String!").unwrap().into_raw() as *const c_char
            ),
            FormatStringResult::NonConstant
        );
    }

    #[cfg(disallow_dangerous_printf)]
    #[test]
    #[should_panic]
    fn test_check_basic_n_directive() {
        _ = panic::take_hook();
        check_format_string("%n\0".as_ptr() as *const c_char);
    }

    #[cfg(not(disallow_dangerous_printf))]
    #[test]
    fn test_check_basic_n_directive() {
        check_format_string("%n\0".as_ptr() as *const c_char);
    }

    #[cfg(disallow_dangerous_printf)]
    #[test]
    #[should_panic]
    fn test_check_complex_n_directive() {
        _ = panic::take_hook();
        check_format_string("%1$hhn\0".as_ptr() as *const c_char);
    }

    #[cfg(not(disallow_dangerous_printf))]
    #[test]
    fn test_check_complex_n_directive() {
        check_format_string("%1$hhn\0".as_ptr() as *const c_char);
    }
}

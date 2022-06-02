use crate::utils::get_ptr_info;
use libc::{c_char, c_int, c_void, dlsym, FILE, RTLD_NEXT};
use std::{
    ffi::{CStr, CString, VaList},
    mem, panic,
};

extern "C" {
    static stdout: *mut FILE;
}

/*
    Hooks vfprintf
    - if the format string is non-constant, replace with a safe version
    - if the format string contains disallowed directives, panic
    - calls vfprintf in glibc with modified arguments to mitigate security risks
*/
#[no_mangle]
pub unsafe extern "C" fn vfprintf(stream: *mut FILE, format: *const c_char, ap: VaList) -> c_int {
    let page_info = get_ptr_info(format as *const c_void).expect("invalid format string pointer");

    if page_info.execute || !page_info.read {
        panic!("invalid format string permissions");
    }

    if page_info.file == Some("[stack]".to_string())
        || page_info.file == Some("[heap]".to_string())
        || page_info.write
    {
        let real_fprintf: extern "C" fn(*mut FILE, *const c_char, ...) -> c_int = mem::transmute(
            dlsym(RTLD_NEXT, CString::new("fprintf").unwrap().into_raw()),
        );
        return real_fprintf(stream, CString::new("%s").unwrap().into_raw(), format);
    }

    if cfg!(disallow_dangerous_printf) {
        let s = CStr::from_ptr(format)
            .to_str()
            .expect("invalid format string");
        if s.contains("%n") {
            panic!("dangerous format string prohibited");
        }
    }

    let real_vfprintf: extern "C" fn(*mut FILE, *const c_char, VaList) -> c_int = mem::transmute(
        dlsym(RTLD_NEXT, CString::new("vfprintf").unwrap().into_raw()),
    );

    real_vfprintf(stream, format, ap)
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

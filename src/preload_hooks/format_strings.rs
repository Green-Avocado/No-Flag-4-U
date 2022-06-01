use crate::utils::get_ptr_info;
use libc::{c_char, c_int, c_void, dlsym, RTLD_NEXT};
use std::{
    ffi::{CStr, CString, VaList},
    mem, panic,
};

/*
    Hooks vprintf
    - if the format string is non-constant, replace with a safe version
    - if the format string contains disallowed directives, panic
    - calls printf in glibc with modified arguments to mitigate security risks
*/
#[no_mangle]
pub unsafe extern "C" fn vprintf(format: *const c_char, ap: VaList) -> c_int {
    let page_info = get_ptr_info(format as *const c_void).expect("invalid format string pointer");

    if page_info.execute || !page_info.read {
        panic!("invalid format string permissions");
    }

    if page_info.file == Some("[stack]".to_string())
        || page_info.file == Some("[heap]".to_string())
        || page_info.write
    {
        let real_printf: extern "C" fn(*const c_char, ...) -> c_int =
            mem::transmute(dlsym(RTLD_NEXT, CString::new("printf").unwrap().into_raw()));
        return real_printf(CString::new("%s").unwrap().into_raw(), format);
    }

    if cfg!(disallow_dangerous_printf) {
        let s = CStr::from_ptr(format)
            .to_str()
            .expect("invalid format string");
        if s.contains("%n") {
            panic!("dangerous format string prohibited");
        }
    }

    let real_vprintf: extern "C" fn(*const c_char, VaList) -> c_int = mem::transmute(dlsym(
        RTLD_NEXT,
        CString::new("vprintf").unwrap().into_raw(),
    ));

    real_vprintf(format, ap)
}

/*
    Hooks printf
    - passes call to vprintf
*/
#[no_mangle]
pub unsafe extern "C" fn printf(format: *const c_char, mut args: ...) -> c_int {
    vprintf(format, args.as_va_list())
}

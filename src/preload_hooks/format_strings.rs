use crate::utils::get_ptr_info;
use libc::{c_char, c_void, dlsym, RTLD_NEXT};
use std::{
    ffi::{CStr, CString},
    mem, panic,
};

// TODO: hook vprintf and use printf as a wrapper for vprintf

/*
    Hooks printf
    - if the format string is non-constant, replace with a safe version
    - if the format string contains disallowed directives, panic
    - calls printf in glibc with modified arguments to mitigate security risks

    TODO: increase limit of args (currently <= 6)
    TODO: enforce limit on args (currently continues to read from the stack)
*/
#[no_mangle]
pub unsafe extern "C" fn printf(mut format: *const c_char, mut args: ...) {
    let mut arg2: usize = args.arg();

    let page_info = get_ptr_info(format as *const c_void).expect("invalid format string pointer");

    if page_info.execute || !page_info.read {
        panic!("invalid format string permissions");
    }

    if page_info.file == Some("[stack]".to_string())
        || page_info.file == Some("[heap]".to_string())
        || page_info.write
    {
        arg2 = format as usize;
        format = CString::new("%s").unwrap().into_raw();
    }

    if cfg!(disallow_dangerous_printf) {
        let s = CStr::from_ptr(format)
            .to_str()
            .expect("invalid format string");
        if s.contains("%n") {
            panic!("dangerous format string prohibited");
        }
    }

    let real_sym: extern "C" fn(*const c_char, ...) =
        mem::transmute(dlsym(RTLD_NEXT, CString::new("printf").unwrap().into_raw()));

    real_sym(
        format,
        arg2,
        args.arg::<usize>(),
        args.arg::<usize>(),
        args.arg::<usize>(),
        args.arg::<usize>(),
        args.arg::<usize>(),
        args.arg::<usize>(),
    );
}

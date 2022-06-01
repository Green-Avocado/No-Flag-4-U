use crate::utils::get_ptr_info;
use libc::{c_char, c_void, dlsym, RTLD_NEXT};
use std::{
    arch::asm,
    ffi::{CStr, CString},
    panic,
};

#[no_mangle]
pub unsafe extern "C" fn printf(mut format: *const c_char, mut args: ...) {
    let mut arg2: usize = args.arg();
    let arg3: usize = args.arg();
    let arg4: usize = args.arg();
    let arg5: usize = args.arg();
    let arg6: usize = args.arg();

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

    let real_sym = dlsym(RTLD_NEXT, CString::new("printf").unwrap().into_raw());

    asm!(
        "leave",
        "jmp rax",
        in("rax") real_sym,
        in("rdi") format,
        in("rsi") arg2,
        in("rdx") arg3,
        in("rcx") arg4,
        in("r8") arg5,
        in("r9") arg6,
        options(noreturn),
    );
}

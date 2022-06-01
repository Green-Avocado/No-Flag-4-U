use crate::{utils::get_ptr_info, LIBC_PATH};
use libc::{c_char, c_void, dlclose, dlopen, dlsym, RTLD_LAZY, RTLD_LOCAL};
use std::{
    arch::asm,
    ffi::{CStr, CString},
    panic,
};

#[no_mangle]
pub extern "C" fn printf(format: *const c_char) {
    let mut rdi: usize;
    let mut rsi: usize;
    let rdx: usize;
    let rcx: usize;
    let r8: usize;
    let r9: usize;

    unsafe {
        asm!(
            "nop",
            out("rdi") rdi,
            out("rsi") rsi,
            out("rdx") rdx,
            out("rcx") rcx,
            out("r8") r8,
            out("r9") r9,
        );
    }

    let page_info =
        get_ptr_info(format as *const _ as *const c_void).expect("invalid format string pointer");

    if page_info.file == Some("[stack]".to_string())
        || page_info.file == Some("[heap]".to_string())
        || !page_info.read
        || page_info.write
        || page_info.execute
    {
        rsi = rdi;
        rdi = CString::new("%s").unwrap().into_raw() as usize;
    }

    if cfg!(disallow_dangerous_printf) {
        let s = unsafe { CStr::from_ptr(rdi as *const c_char) }
            .to_str()
            .expect("invalid format string");
        if s.contains("%n") {
            panic!("dangerous format string prohibited");
        }
    }

    unsafe {
        let handle = dlopen(
            CString::new(LIBC_PATH).unwrap().into_raw(),
            RTLD_LAZY | RTLD_LOCAL,
        );
        let real_sym = dlsym(handle, CString::new("printf").unwrap().into_raw());
        dlclose(handle);

        asm!(
            "leave",
            "jmp rax",
            in("rax") real_sym,
            in("rdi") rdi,
            in("rsi") rsi,
            in("rdx") rdx,
            in("rcx") rcx,
            in("r8") r8,
            in("r9") r9,
        );
    }

    unreachable!();
}

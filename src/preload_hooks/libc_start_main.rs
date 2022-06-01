use crate::{LIBC_PATH, MAIN_STARTED};
use libc::{dlclose, dlopen, dlsym, RTLD_LAZY, RTLD_LOCAL};
use std::{arch::asm, ffi::CString, panic, process::exit, sync::atomic::Ordering};

#[no_mangle]
pub extern "C" fn __libc_start_main() {
    let rdi: usize;
    let rsi: usize;
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

    if !cfg!(debug_assertions) {
        panic::set_hook(Box::new(|_| {
            exit(-1);
        }));
    }

    unsafe {
        let handle = dlopen(
            CString::new(LIBC_PATH).unwrap().into_raw(),
            RTLD_LAZY | RTLD_LOCAL,
        );
        let real_sym = dlsym(
            handle,
            CString::new("__libc_start_main").unwrap().into_raw(),
        );
        dlclose(handle);

        MAIN_STARTED.store(true, Ordering::SeqCst);

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

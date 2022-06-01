use crate::MAIN_STARTED;
use libc::{c_char, c_int, dlsym, RTLD_NEXT};
use std::{arch::asm, ffi::CString, panic, process::exit, sync::atomic::Ordering};

#[no_mangle]
pub unsafe extern "C" fn __libc_start_main(
    main: extern "C" fn(c_int, *const *const c_char, *const *const c_char) -> c_int,
    arg2: usize,
    arg3: usize,
    arg4: usize,
    arg5: usize,
    arg6: usize,
) {
    if !cfg!(debug_assertions) {
        panic::set_hook(Box::new(|_| {
            exit(-1);
        }));
    }

    let real_sym = dlsym(
        RTLD_NEXT,
        CString::new("__libc_start_main").unwrap().into_raw(),
    );

    MAIN_STARTED.store(true, Ordering::SeqCst);

    asm!(
        "leave",
        "jmp rax",
        in("rax") real_sym,
        in("rdi") main,
        in("rsi") arg2,
        in("rdx") arg3,
        in("rcx") arg4,
        in("r8") arg5,
        in("r9") arg6,
        options(noreturn),
    );
}

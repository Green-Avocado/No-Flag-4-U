use crate::MAIN_STARTED;
use libc::{c_char, c_int, dlsym, RTLD_NEXT};
use std::{ffi::CString, mem, panic, process::exit, sync::atomic::Ordering};

#[no_mangle]
pub unsafe extern "C" fn __libc_start_main(
    main: extern "C" fn(c_int, *const *const c_char, *const *const c_char) -> c_int,
    arg2: usize,
    arg3: usize,
    arg4: usize,
    arg5: usize,
    arg6: usize,
    arg7: usize,
    arg8: usize,
) {
    if !cfg!(debug_assertions) {
        panic::set_hook(Box::new(|_| {
            exit(-1);
        }));
    }

    MAIN_STARTED.store(true, Ordering::SeqCst);

    let real_sym: extern "C" fn(
        main: extern "C" fn(c_int, *const *const c_char, *const *const c_char) -> c_int,
        arg2: usize,
        arg3: usize,
        arg4: usize,
        arg5: usize,
        arg6: usize,
        arg7: usize,
        arg8: usize,
    ) = mem::transmute(dlsym(
        RTLD_NEXT,
        CString::new("__libc_start_main").unwrap().into_raw(),
    ));
    real_sym(main, arg2, arg3, arg4, arg5, arg6, arg7, arg8);
}

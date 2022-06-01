use crate::MAIN_STARTED;
use libc::{c_char, c_int, dlsym, RTLD_NEXT};
use std::{ffi::CString, mem, panic, process::exit, sync::atomic::Ordering};

/*
    Hooks __libc_start_main
    - sets MAIN_STARTED so other functions can switch to safe mode
    - consumes 8 arguments in case glibc is compiled with LIBC_START_MAIN_AUXVEC_ARG
    - calls __libc_start_main in glibc with the original arguments
*/
#[no_mangle]
pub unsafe extern "C" fn __libc_start_main(
    main: extern "C" fn(c_int, *const *const c_char, *const *const c_char) -> c_int,
    mut args: ...
) -> c_int {
    if !cfg!(debug_assertions) {
        panic::set_hook(Box::new(|_| {
            exit(-1);
        }));
    }

    MAIN_STARTED.store(true, Ordering::SeqCst);

    let real_libc_start_main: extern "C" fn(
        extern "C" fn(c_int, *const *const c_char, *const *const c_char) -> c_int,
        ...
    ) -> c_int = mem::transmute(dlsym(
        RTLD_NEXT,
        CString::new("__libc_start_main").unwrap().into_raw(),
    ));

    real_libc_start_main(
        main,
        args.arg::<usize>(),
        args.arg::<usize>(),
        args.arg::<usize>(),
        args.arg::<usize>(),
        args.arg::<usize>(),
        args.arg::<usize>(),
        args.arg::<usize>(),
    )
}

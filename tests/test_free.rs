use libc::c_void;
use std::panic;
use somewhat_safe_glibc_wrappers::preload_hooks::memory_management::free;

#[test]
fn test_zero() {
    unsafe { free(0 as *mut c_void) };
}

#[test]
#[should_panic]
fn test_invalid() {
    _ = panic::take_hook();
    unsafe { free(1 as *mut c_void) };
}

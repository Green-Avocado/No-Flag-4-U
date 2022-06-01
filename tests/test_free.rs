use libc::c_void;
use super_safe_glibc_wrappers::preload_hooks::memory_management::free;

#[test]
fn test_free_zero() {
    free(0 as *mut c_void);
}

#[test]
#[should_panic]
fn test_free_invalid() {
    free(1 as *mut c_void);
}

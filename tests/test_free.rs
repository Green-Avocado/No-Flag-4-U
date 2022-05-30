use libc::c_void;
use super_safe_glibc_wrappers::free;

#[test]
fn test_free() {
    free(0 as *mut c_void);
    assert!(true);
}

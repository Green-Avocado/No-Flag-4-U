use libc::c_char;
use std::ffi::CString;
use super_safe_glibc_wrappers::printf;

#[test]
fn test_printf_normal() {
    printf(CString::new("Hello, world!").unwrap().into_raw());
}

#[test]
#[should_panic]
fn test_free_invalid() {
    printf(1 as *const c_char);
}

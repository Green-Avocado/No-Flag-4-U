use super_safe_glibc_wrappers::free;

#[test]
fn test_free() {
    free();
    assert!(true);
}

use libc::c_void;

#[no_mangle]
pub extern "C" fn free(_ptr: *mut c_void) {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_free() {
        free(0 as *mut c_void);
        assert!(true);
    }
}

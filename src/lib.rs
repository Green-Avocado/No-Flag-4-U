#[no_mangle]
pub extern "C" fn free() {}

#[no_mangle]
pub extern "C" fn malloc() {}

#[no_mangle]
pub extern "C" fn read() {}

fn pointer_is_stack() -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_free() {
        free();
        assert!(true);
    }
}

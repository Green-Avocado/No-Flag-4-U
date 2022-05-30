#[no_mangle]
pub extern "C" fn free() {
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

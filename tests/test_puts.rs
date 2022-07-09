#[cfg(not(feature = "disable_puts_hooks"))]
mod tests {
    use inline_c::assert_c;
    use std::panic;

    #[test]
    fn test_normal() {
        (assert_c! {
            #include <stdio.h>

            int main() {
                puts("Hello, world!");
                return 0;
            }
        })
        .success()
        .stdout("Hello, world!\n");
    }
}

#[cfg(not(disable_write_hooks))]
mod tests {
    use inline_c::assert_c;
    use std::panic;

    #[test]
    fn test_normal() {
        (assert_c! {
            #include <unistd.h>

            int main() {
                write(1, "Hello, world!\n", 14);
                return 0;
            }
        })
        .success()
        .stdout("Hello, world!\n");
    }
}

#[cfg(not(disable_format_string_hooks))]
mod tests {
    use inline_c::assert_c;
    use libc::c_char;
    use somewhat_safe_glibc_wrappers::preload_hooks::format_strings::printf;
    use std::panic;

    #[test]
    fn test_normal() {
        (assert_c! {
            #include <stdio.h>

            int main() {
                printf("Hello, world!\n");
                return 0;
            }
        })
        .success()
        .stdout("Hello, world!\n");
    }

    #[test]
    fn test_6_directives() {
        (assert_c! {
            #include <stdio.h>

            int main() {
                printf(
                    "%d\n%c\n%lx\n%05u\n%s\n%.2f\n",
                    13, 'c', (unsigned long) -1, 11, "Test", 1.1);
                return 0;
            }
        })
        .success()
        .stdout(
            "13\n\
            c\n\
            ffffffffffffffff\n\
            00011\n\
            Test\n\
            1.10\n",
        );
    }

    #[test]
    fn test_heap_and_stack() {
        (assert_c! {
            #include <stdio.h>
            #include <stdlib.h>
            #include <string.h>

            int main() {
                char* s = malloc(8);
                strncpy(s, "%p\n", 8);
                printf(s);
                return 0;
            }
        })
        .success()
        .stdout("%p\n");

        (assert_c! {
            #include <stdio.h>
            #include <string.h>

            int main() {
                char s[8];
                strncpy(s, "%p\n", 8);
                printf(s);
                return 0;
            }
        })
        .success()
        .stdout("%p\n");
    }

    #[test]
    fn test_n_directives() {
        let mut assertion_simple = assert_c! {
            #include <stdio.h>
            #include <string.h>

            int main() {
                int n;
                printf("Hello, world!%n", &n);
                return 0;
            }
        };

        let mut assertion_complex = assert_c! {
            #include <stdio.h>
            #include <string.h>

            int main() {
                char n;
                printf("Test String%1$hhn", &n);
                return 0;
            }
        };

        if cfg!(enable_n_directive_filter) {
            assertion_simple.failure();
            assertion_complex.failure();
        } else {
            assertion_simple.success();
            assertion_complex.success();
        }
    }

    #[test]
    #[should_panic]
    fn test_invalid() {
        _ = panic::take_hook();
        unsafe { printf(1 as *const c_char) };
    }
}

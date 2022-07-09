#[cfg(not(feature = "disable_heap_hooks"))]
mod tests {
    use inline_c::assert_c;
    use libc::c_void;
    use no_flag_4_u::preload_hooks::heap::free;

    use std::panic;

    #[test]
    fn test_zero() {
        unsafe { free(0 as *mut c_void) };
    }

    #[test]
    #[should_panic]
    fn test_invalid() {
        _ = panic::take_hook();
        unsafe { free(1 as *mut c_void) };
    }

    #[test]
    fn test_normal() {
        (assert_c! {
            #include <stdio.h>
            #include <stdlib.h>
            #include <string.h>

            int main() {
                char* buf = malloc(0x10);
                strncpy(buf, "Hello, world!", 0x10);
                puts(buf);
                free(buf);
                return 0;
            }
        })
        .success();
    }

    #[test]
    fn test_stack() {
        (assert_c! {
            #include <stdio.h>
            #include <stdlib.h>
            #include <string.h>

            int main() {
                char buf[0x10];
                strncpy(buf, "Hello, world!", 0x10);
                puts(buf);
                free(buf);
                return 0;
            }
        })
        .failure();
    }

    #[test]
    fn test_executable() {
        (assert_c! {
            #include <stdio.h>
            #include <stdlib.h>
            #include <string.h>

            int main() {
                free(main);
                return 0;
            }
        })
        .failure();
    }
}

#[cfg(not(disable_heap_hooks))]
mod tests {
    use inline_c::assert_c;
    use libc::c_void;
    use somewhat_safe_glibc_wrappers::preload_hooks::heap::free;
    use std::env::{remove_var, set_var};
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
        set_var("INLINE_C_RS_CFLAGS", "-Wno-error=free-nonheap-object");

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

        remove_var("INLINE_C_RS_CFLAGS");
    }

    #[test]
    fn test_executable() {
        set_var("INLINE_C_RS_CFLAGS", "-Wno-error=free-nonheap-object");

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

        remove_var("INLINE_C_RS_CFLAGS");
    }
}

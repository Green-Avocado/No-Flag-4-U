use inline_c::assert_c;
use libc::c_char;
use std::{ffi::CString, panic};
use super_safe_glibc_wrappers::preload_hooks::format_strings::printf;

#[test]
fn test_printf_normal() {
    unsafe { printf(CString::new("Hello, world!\n").unwrap().into_raw()) };
}

#[test]
fn test_printf_stdout_normal() {
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
fn test_printf_stdout_heap() {
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
}

#[test]
#[should_panic]
fn test_free_invalid() {
    _ = panic::take_hook();
    unsafe { printf(1 as *const c_char) };
}

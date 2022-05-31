#![feature(iter_advance_by)]
#![feature(panic_always_abort)]

use libc::{c_char, c_void, dlclose, dlopen, dlsym, RTLD_LAZY, RTLD_LOCAL};
use std::{arch::asm, ffi::CString, fs, panic, process::exit};

static mut MAIN_STARTED: bool = false;
static mut FREE_RECURSION_GUARD: bool = true;

struct PageInfo {
    read: bool,
    write: bool,
    execute: bool,
    file: Option<String>,
}

#[no_mangle]
pub extern "C" fn __libc_start_main() {
    let rdi: usize;
    let rsi: usize;
    let rdx: usize;
    let rcx: usize;
    let r8: usize;
    let r9: usize;

    unsafe {
        asm!(
            "nop",
            out("rdi") rdi,
            out("rsi") rsi,
            out("rdx") rdx,
            out("rcx") rcx,
            out("r8") r8,
            out("r9") r9,
        );
    }

    if !cfg!(debug_assertions) {
        panic::set_hook(Box::new(|_| {
            exit(-1);
        }));
    }

    unsafe {
        let handle = dlopen(
            CString::new("/lib/libc.so.6").unwrap().into_raw(),
            RTLD_LAZY | RTLD_LOCAL,
        );
        let real_sym = dlsym(
            handle,
            CString::new("__libc_start_main").unwrap().into_raw(),
        );
        dlclose(handle);

        MAIN_STARTED = true;

        asm!(
            "leave",
            "jmp rax",
            in("rax") real_sym,
            in("rdi") rdi,
            in("rsi") rsi,
            in("rdx") rdx,
            in("rcx") rcx,
            in("r8") r8,
            in("r9") r9,
        );
    }
}

#[no_mangle]
pub extern "C" fn free(ptr: *mut c_void) {
    if ptr as usize == 0 {
        return;
    }

    unsafe {
        if !FREE_RECURSION_GUARD {
            return;
        }

        FREE_RECURSION_GUARD = false;
    }

    if unsafe { !MAIN_STARTED } {
        unsafe {
            let handle = dlopen(
                CString::new("/lib/libc.so.6").unwrap().into_raw(),
                RTLD_LAZY | RTLD_LOCAL,
            );
            let real_sym = dlsym(handle, CString::new("free").unwrap().into_raw());
            dlclose(handle);

            asm!(
                "call rax",
                in("rax") real_sym,
                in("rdi") ptr,
            );
        }
    } else {
        let page_info = get_ptr_info(ptr).expect("freeing invalid pointer");

        if !(page_info.read && page_info.write && !page_info.execute) {
            panic!("freeing invalid permissions");
        }

        if page_info.file == Some("[stack]".to_string()) {
            panic!("freeing in stack");
        }
    }

    unsafe {
        FREE_RECURSION_GUARD = true;
    }
}

#[no_mangle]
pub extern "C" fn printf(format: *const c_char) {
    let rdi: usize;
    let rsi: usize;
    let rdx: usize;
    let rcx: usize;
    let r8: usize;
    let r9: usize;

    unsafe {
        asm!(
            "nop",
            out("rdi") rdi,
            out("rsi") rsi,
            out("rdx") rdx,
            out("rcx") rcx,
            out("r8") r8,
            out("r9") r9,
        );
    }

    let page_info =
        get_ptr_info(format as *const _ as *const c_void).expect("invalid format string pointer");

    if page_info.file == Some("[stack]".to_string()) {
        panic!("format string in stack");
    }

    if page_info.file == Some("[heap]".to_string()) {
        panic!("format string in heap");
    }

    if !page_info.read || page_info.write || page_info.execute {
        panic!("invalid permissions for format string");
    }

    unsafe {
        let handle = dlopen(
            CString::new("/lib/libc.so.6").unwrap().into_raw(),
            RTLD_LAZY | RTLD_LOCAL,
        );
        let real_sym = dlsym(handle, CString::new("printf").unwrap().into_raw());
        dlclose(handle);

        asm!(
            "leave",
            "jmp rax",
            in("rax") real_sym,
            in("rdi") rdi,
            in("rsi") rsi,
            in("rdx") rdx,
            in("rcx") rcx,
            in("r8") r8,
            in("r9") r9,
        );
    }
}

fn get_ptr_info(ptr: *const c_void) -> Option<PageInfo> {
    const PARSE_ERR: &str = "failed to parse maps";

    let contents = fs::read_to_string("/proc/self/maps").expect("could not read /proc/self/maps");

    for row in contents.lines() {
        let mut columns = row.split_whitespace();
        let bounds = columns
            .next()
            .expect(PARSE_ERR)
            .split_once('-')
            .expect(PARSE_ERR);

        let lower_bound = usize::from_str_radix(bounds.0, 16).expect(PARSE_ERR);
        let upper_bound = usize::from_str_radix(bounds.1, 16).expect(PARSE_ERR);

        if lower_bound <= ptr as usize && ptr as usize <= upper_bound {
            let mut chars = columns.next().expect(PARSE_ERR).chars();

            let read = 'r' == chars.next().expect(PARSE_ERR);
            let write = 'w' == chars.next().expect(PARSE_ERR);
            let execute = 'x' == chars.next().expect(PARSE_ERR);

            columns.advance_by(3).expect(PARSE_ERR);
            let file = columns.next().map(str::to_string);

            let rsp: usize;
            unsafe {
                asm!("mov {}, rsp", out(reg) rsp);
            }

            if file == Some("[stack]".to_string()) && rsp < ptr as usize {
                panic!("dangling stack pointer");
            }

            return Some(PageInfo {
                read,
                write,
                execute,
                file,
            });
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_ptr_info_invalid() {
        assert!(get_ptr_info(1 as *const c_void).is_none());
    }

    #[test]
    fn test_get_ptr_info_const() {
        const TEST_VALUE: u64 = 0x1337;
        let page_info = get_ptr_info(&TEST_VALUE as *const _ as *const c_void).unwrap();
        assert_eq!(page_info.read, true);
        assert_eq!(page_info.write, false);
        assert_eq!(page_info.execute, false);
    }
}

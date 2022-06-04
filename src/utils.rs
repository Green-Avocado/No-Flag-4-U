use libc::{c_void, dlsym, RTLD_NEXT};
use std::{arch::asm, ffi::CString, fs};
use zeroize::Zeroize;

pub struct PageInfo {
    pub read: bool,
    pub write: bool,
    pub execute: bool,
    pub file: Option<String>,
}

/*
    Wrapps dlsym() to get the next pointer for a symbol
*/
pub unsafe fn dlsym_next(symbol: &str) -> *mut c_void {
    dlsym(RTLD_NEXT, CString::new(symbol).unwrap().into_raw())
}

/*
    Parses /proc/self/maps to return information about the page a pointer resides in
*/
pub fn get_ptr_info(ptr: *const c_void) -> Option<PageInfo> {
    const PARSE_ERR: &str = "failed to parse maps";
    let mut page_info = None;

    let mut contents =
        fs::read_to_string("/proc/self/maps").expect("could not read /proc/self/maps");

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

            if file == Some("[stack]".to_string()) && rsp > ptr as usize {
                panic!("dangling stack pointer");
            }

            page_info = Some(PageInfo {
                read,
                write,
                execute,
                file,
            });
        }
    }

    contents.zeroize();

    page_info
}

pub fn log(_info: &str) {
    // TODO: log to file
    // log using a separate process to allow seccomp protections and fault tolerance
    // communicate via socket
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{arch::asm, boxed::Box};

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

    #[test]
    fn test_get_ptr_info_stack() {
        let rsp: usize;
        unsafe {
            asm!("mov {}, rsp", out(reg) rsp);
        }
        let page_info = get_ptr_info(rsp as *const c_void).unwrap();
        assert_eq!(page_info.read, true);
        assert_eq!(page_info.write, true);
        assert_eq!(page_info.execute, false);
    }

    #[test]
    fn test_get_ptr_info_dynamic() {
        let test_value = Box::new(0x1337);
        let page_info = get_ptr_info(Box::into_raw(test_value) as *const c_void).unwrap();
        assert_eq!(page_info.read, true);
        assert_eq!(page_info.write, true);
        assert_eq!(page_info.execute, false);
    }
}

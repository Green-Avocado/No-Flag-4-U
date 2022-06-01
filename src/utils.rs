use libc::{c_void};
use std::{
    arch::asm,
    fs, panic,
    sync::atomic::{AtomicBool},
};
use zeroize::Zeroize;

pub struct PageInfo {
    pub read: bool,
    pub write: bool,
    pub execute: bool,
    pub file: Option<String>,
}

pub fn get_ptr_info(ptr: *const c_void) -> Option<PageInfo> {
    const PARSE_ERR: &str = "failed to parse maps";
    let mut ret = None;

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

            ret = Some(PageInfo {
                read,
                write,
                execute,
                file,
            });
        }
    }

    contents.zeroize();

    ret
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

use libc::c_void;
use std::fs;

struct PageInfo {
    read: bool,
    write: bool,
    execute: bool,
}

#[no_mangle]
pub extern "C" fn free(ptr: *mut c_void) {
    get_ptr_info(ptr);
}

fn get_ptr_info(ptr: *mut c_void) -> PageInfo {
    const PARSE_ERR: &str = "failed to parse maps";

    let contents = fs::read_to_string("/proc/self/maps").expect("could not read /proc/self/maps");

    for row in contents.lines() {
        let mut columns = row.split_whitespace();
        let bounds = columns
            .next()
            .expect(PARSE_ERR)
            .split_once('-')
            .expect(PARSE_ERR);

        let lower_bound = bounds.0.parse::<usize>().expect(PARSE_ERR);
        let upper_bound = bounds.1.parse::<usize>().expect(PARSE_ERR);

        if lower_bound <= ptr as usize && ptr as usize <= upper_bound {
            let mut chars = columns.next().expect(PARSE_ERR).chars();

            let read = if 'r' == chars.next().expect(PARSE_ERR) {
                true
            } else {
                false
            };

            let write = if 'w' == chars.next().expect(PARSE_ERR) {
                true
            } else {
                false
            };

            let execute = if 'x' == chars.next().expect(PARSE_ERR) {
                true
            } else {
                false
            };

            return PageInfo {
                read,
                write,
                execute,
            };
        }
    }

    panic!("{}", PARSE_ERR);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_free() {
        free(0 as *mut c_void);
        assert!(true);
    }

    #[test]
    fn test_get_ptr_info() {
        get_ptr_info(0 as *mut c_void);
        assert!(true);
    }
}

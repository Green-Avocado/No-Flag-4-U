use crate::{utils::get_ptr_info, LIBC_PATH, MAIN_STARTED};
use libc::{c_void, dlclose, dlopen, dlsym, RTLD_LAZY, RTLD_LOCAL};
use std::{arch::asm, cell::Cell, ffi::CString, panic, sync::atomic::Ordering};

thread_local! {
    static FREE_RECURSION_GUARD: Cell<bool> = Cell::new(true);
}

#[no_mangle]
pub extern "C" fn free(ptr: *mut c_void) {
    if ptr as usize == 0 {
        return;
    }

    if !FREE_RECURSION_GUARD.get() {
        return;
    }

    FREE_RECURSION_GUARD.set(false);

    if !MAIN_STARTED.load(Ordering::SeqCst) {
        unsafe {
            let handle = dlopen(
                CString::new(LIBC_PATH).unwrap().into_raw(),
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

    FREE_RECURSION_GUARD.set(true);
}

mod libc_start_main;

#[cfg(not(disable_fmtstr_hooks))]
pub mod format_strings;

#[cfg(not(disable_heap_hooks))]
pub mod heap;

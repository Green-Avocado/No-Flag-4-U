mod libc_start_main;

#[cfg(not(disable_read_hooks))]
pub mod read;

#[cfg(not(disable_write_hooks))]
pub mod write;

#[cfg(not(disable_gets_hooks))]
pub mod gets;

#[cfg(not(disable_puts_hooks))]
pub mod puts;

#[cfg(not(disable_scanf_hooks))]
pub mod scanf;

#[cfg(not(disable_printf_hooks))]
pub mod printf;

#[cfg(not(disable_heap_hooks))]
pub mod heap;

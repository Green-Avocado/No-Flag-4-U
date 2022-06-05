#![feature(iter_advance_by)]
#![feature(panic_update_hook)]
#![feature(local_key_cell_methods)]
#![feature(c_variadic)]

mod config;
pub mod preload_hooks;
mod utils;

use std::sync::atomic::AtomicBool;

/// Returns `true` if `__libc_start_main` has been run.
/// Returns `false` otherwise.
static MAIN_STARTED: AtomicBool = AtomicBool::new(false);

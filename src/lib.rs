#![feature(iter_advance_by)]
#![feature(panic_update_hook)]
#![feature(local_key_cell_methods)]
#![feature(c_variadic)]

pub mod preload_hooks;
mod utils;

use std::sync::atomic::AtomicBool;

static MAIN_STARTED: AtomicBool = AtomicBool::new(false);

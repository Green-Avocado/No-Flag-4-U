#![feature(iter_advance_by)]
#![feature(panic_always_abort)]
#![feature(local_key_cell_methods)]

pub mod format_strings;
pub mod libc_start_main;
pub mod memory_management;
mod utils;

use libc::{c_void};
use std::{
    arch::asm,
    fs, panic,
    sync::atomic::{AtomicBool},
};
use zeroize::Zeroize;

const LIBC_PATH: &str = "/usr/lib/libc.so.6";
static MAIN_STARTED: AtomicBool = AtomicBool::new(false);

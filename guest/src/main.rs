#![no_std]
#![no_main]
#![feature(panic_info_message, global_asm, asm)]

// extenal crates
extern crate log;

// modules
#[macro_use]
pub mod uart;
pub mod boot;
mod debug;
pub mod kernel;
pub mod memlayout;
pub mod paging;

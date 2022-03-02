#![no_std]
#![no_main]

#![feature(panic_info_message, type_ascription, asm_const)]

// extenal crates
extern crate elf_rs;
extern crate log;

// modules
#[macro_use]
pub mod uart;
#[macro_use]
pub mod riscv;
pub mod boot;
pub mod memlayout;
pub mod paging;
pub mod plic;
pub mod clint;

pub mod mkernel;

pub mod guest;
pub mod hypervisor;

pub mod debug;
pub mod util;

pub mod virtio;

#![no_std]
#![no_main]

#![feature(panic_info_message, type_ascription, asm_const)]
#![feature(default_alloc_error_handler)]


// Heap implementation
use buddy_system_allocator::LockedHeap;

#[global_allocator]
static HEAP_ALLOCATOR: LockedHeap<32> = LockedHeap::empty();

// extenal crates
extern crate elf_rs;
extern crate log;
extern crate alloc;

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
pub mod count_harts;

pub mod mkernel;

pub mod guest;
pub mod hypervisor;

pub mod debug;
pub mod util;

pub mod virtio;

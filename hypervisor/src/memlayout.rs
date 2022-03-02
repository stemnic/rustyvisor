// generic constants
/////

pub const PAGE_SIZE: u16 = 4096;
pub const VIRTIO0_IRQ: u16 = 1;
pub const UART0_IRQ: u16 = 10;

// information on hypervisor binary
/////
extern "C" {
    static _elf_start: usize;
    static _elf_end: usize;
}

pub unsafe fn elf_start() -> usize {
    unsafe { &_elf_start as *const usize as usize }
}

pub unsafe fn elf_end() -> usize {
    unsafe { &_elf_end as *const usize as usize }
}

// information on hardware for hypervisor
/////

pub static UART_BASE: usize = 0x1000_0000;
pub static VIRTIO0_BASE: usize = 0x1000_1000;

pub static PLIC_BASE: usize = 0x0c00_0000;
pub static PLIC_HART0_M_INT_EN: usize = 0x2000;
pub static PLIC_HART0_M_PRIORITY: usize = 0x20_0000;
pub static PLIC_HART0_M_CLAIM_COMPLETE: usize = 0x20_0004;
pub static PLIC_HART1_M_INT_EN: usize = 0x2080;
pub static PLIC_HART1_M_PRIORITY: usize = 0x20_1000;
pub static PLIC_HART1_M_CLAIM_COMPLETE: usize = 0x20_1004;

pub static CLINT_HART0_MSIP: usize = 0x200_0000;
pub static CLINT_HART1_MSIP: usize = 0x200_0004;
pub static CLINT_HART2_MSIP: usize = 0x200_0008;
pub static CLINT_HART3_MSIP: usize = 0x200_000c;
pub static CLINT_HART4_MSIP: usize = 0x200_0010;

pub static CLINT_HART0_MTIMECMP: usize = 0x200_4000;
pub static CLINT_HART1_MTIMECMP: usize = 0x200_4008;
pub static CLINT_HART2_MTIMECMP: usize = 0x200_4010;
pub static CLINT_HART3_MTIMECMP: usize = 0x200_4018;
pub static CLINT_HART4_MTIMECMP: usize = 0x200_4020;

pub static CLINT_MTIME: usize = 0x200_bff8;


// TODO: make this more flexible
// This value should be page-aligned.
pub static DRAM_START: usize = 0x8000_0000;
pub static DRAM_END: usize = 0x9000_0000;

// information on hardware for guest
/////

pub static GUEST_UART_BASE: usize = 0x1000_0000;

// TODO: make this more flexible
// This value should be page-aligned.
pub static GUEST_DRAM_START: usize = 0x8000_0000;
pub static GUEST_DRAM_END: usize = 0x8200_0000;

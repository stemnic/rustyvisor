// generic constants
/////

pub const PAGE_SIZE: u16 = 4096;
pub static UART_BASE: usize = 0x1000_0000;
pub static DRAM_START: usize = 0x8000_0000;
pub static DRAM_END: usize = 0x8200_0000;
pub static USER_TEST_START: usize = 0x8201_0000;
pub static USER_TEST_END: usize = 0x8202_0000;

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
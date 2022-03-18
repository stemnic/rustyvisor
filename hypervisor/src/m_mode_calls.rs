/// M mode calls is syscalls to the m mode layer todo tasks that require M mode priviliges

use crate::riscv;

pub const DISABLE_ALL_INTERRUPTS: usize = 0x01;
pub const ENABLE_ALL_INTERRUPTS: usize  = 0x02;
pub const DISABLE_ALL_TIMERS: usize     = 0x03;
pub const ENABLE_ALL_TIMERS: usize      = 0x04;

pub fn disable_interrupts() {
    let _ = riscv::instruction::ecall_with_args(DISABLE_ALL_INTERRUPTS, 0x0, 0x0, 0x0);
}

pub fn enable_interrupts() {
    let _ = riscv::instruction::ecall_with_args(ENABLE_ALL_INTERRUPTS, 0x0, 0x0, 0x0);
}

pub fn disable_timers() {
    let _ = riscv::instruction::ecall_with_args(DISABLE_ALL_TIMERS, 0x0, 0x0, 0x0);
}

pub fn enable_timers() {
    let _ = riscv::instruction::ecall_with_args(ENABLE_ALL_TIMERS, 0x0, 0x0, 0x0);
}
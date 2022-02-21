use core::arch::global_asm;

// include raw assembly codes in ./asm
global_asm!(include_str!("boot.S"));

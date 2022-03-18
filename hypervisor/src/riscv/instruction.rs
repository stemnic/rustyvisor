use core::arch::{asm, global_asm};

global_asm!(include_str!("instruction.S"));

pub fn ecall_with_args(arg0: usize, arg1: usize, arg2: usize, arg3: usize) -> (usize, usize) {
    let (out0, out1);
    match () {
        #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
        () => unsafe { asm!(
            "ecall",
            in("a0") arg0, in("a1") arg1,
            in("a2") arg2, in("a3") arg3,
            lateout("a0") out0, lateout("a1") out1,
        ) },
        #[cfg(not(any(target_arch = "riscv32", target_arch = "riscv64")))]
        () => {
            drop((extension, function, arg0, arg1));
            unimplemented!("not RISC-V instruction set architecture")
        }
    };
    (out0, out1)
}

pub fn mret() -> ! {
    unsafe {
        asm!("mret");
    }

    loop {}
}

pub fn sret() -> ! {
    unsafe {
        asm!("sret");
    }

    loop {}
}

pub fn ecall() {
    unsafe {
        asm!("ecall");
    }
}

pub fn sfenve_vma() {
    unsafe {
        asm!("sfence.vma");
    }
}

extern "C" {
    fn __hfence_gvma_all();
}

pub fn hfence_gvma() {
    unsafe {
        __hfence_gvma_all();
    }
}

pub fn wfi() {
    unsafe {
        asm!("wfi");
    }
}

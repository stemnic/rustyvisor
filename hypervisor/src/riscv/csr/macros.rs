/*
asm!(assembly template
   : output operands
   : input operands
   : clobbers
   : options
   );

*/

macro_rules! define_read {
    ($csr_number:expr) => {
        use core::arch::asm;
        #[inline]
        pub fn read() -> usize {
            unsafe {
                let r: usize;
                asm!("csrrs {0}, {csr}, x0",
                out(reg) r,
                csr = const $csr_number,
                options(nostack)
                );
                /* 
                asm!("csrrs $0, $1, x0" 
                : "=r"(r) 
                : "i"($csr_number) 
                :
                : "volatile");
                */
                r
            }
        }
    };
}

macro_rules! define_write {
    ($csr_number:expr) => {
        pub fn write(v: usize) {
            unsafe {
                asm!("csrrw x0, {csr}, {rs}",
                rs = in(reg) v,
                csr = const $csr_number,
                options(nostack)
                );
                /*
                asm!("csrrw x0, $1, $0" 
                :
                : "r"(v), "i"($csr_number) 
                :
                : "volatile");
                */
            }
        }
    };
}

global_asm!(include_str!("kernel.S"));

use crate::memlayout;
use crate::uart;
use crate::sbi::ecall::{SbiRet, EXTENSION_TIMER};
use core::arch::asm;
use core::arch::global_asm;
use core::fmt::Error;

extern "C" {
    #[link_name = "trap_to_kernel"]
    pub fn trap();
}

#[no_mangle]
pub extern "C" fn rust_entrypoint() -> ! {
    if let Err(e) = init() {
        panic!("failed to initialize. {:?}", e);
    };
    println!("hello world from a guest");

    let call = sbi_call(EXTENSION_TIMER, 0x0, 0xdead, 0xbeef);
    println!("Sbi call error: {},  value: {}", call.error, call.value);

    loop {}
}

fn sbi_call(extension: usize, function: usize, arg0: usize, arg1: usize) -> SbiRet {
    let (error, value);
    match () {
        #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
        () => unsafe { asm!(
            "ecall",
            in("a0") arg0, in("a1") arg1,
            in("a6") function, in("a7") extension,
            lateout("a0") error, lateout("a1") value,
        ) },
        #[cfg(not(any(target_arch = "riscv32", target_arch = "riscv64")))]
        () => {
            drop((extension, function, arg0, arg1));
            unimplemented!("not RISC-V instruction set architecture")
        }
    };
    SbiRet { error, value }
}

pub fn init() -> Result<(), Error> {
    // init UART
    uart::Uart::new(memlayout::UART_BASE).init();

    // leave
    Ok(())
}

#[no_mangle]
pub extern "C" fn rust_trap_handler() {
    log::info!("trapped!");
}

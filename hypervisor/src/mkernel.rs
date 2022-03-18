global_asm!(include_str!("mkernel.S"));

use crate::HEAP_ALLOCATOR;
use crate::clint;
use crate::count_harts;
use crate::hypervisor;
use crate::memlayout;
use crate::memlayout::{heap_start, heap_end, HEAP_SIZE};
use crate::riscv;
use crate::uart;
use crate::util;
use core::arch::asm;
use core::arch::global_asm;
use core::fmt::Error;
extern "C" {
    #[link_name = "trap_to_mkernel"]
    pub fn trap();
}

#[no_mangle]
pub extern "C" fn rust_m_entrypoint(hartid: usize, opqaue: usize) -> ! {
    // init hardware and M-mode registers.
    if let Err(e) = init() {
        panic!("Failed to initialize. {:?}", e);
    };

    println!("-----------------------");
    println!(" rvvisor");
    println!("-----------------------");

    // init logger.
    if let Err(e) = util::logger::init() {
        panic!("Failed to init logger. {:?}", e);
    }
    log::info!("logger was initialized");
    log::info!("processor is in m-mode running with hartid: {}", hartid);
    unsafe {
        log::info!("Initing heap implementation: 0x{:016x} -> 0x{:016x} size: 0x{:016x}", heap_start(), heap_end(), HEAP_SIZE);
        HEAP_ALLOCATOR.lock().add_to_heap(heap_start(), heap_end());
    }


    unsafe { count_harts::init_hart_count(opqaue) };

    //init clint timer
    if let Err(e) = setup_timer() {
        panic!("Failed to initialize timer. {:?}", e);
    };

    // jump to a next handler while changing CPU mode to HS
    log::info!("jump to hypervisor while chainging CPU mode from M to HS");
    switch_to_hypervisor(hypervisor::entrypoint as unsafe extern "C" fn());
}

pub fn setup_timer() -> Result<(), Error> {

    // clint: enable timer interrupts
    let timer = clint::Clint::new(0x200_0000 as *mut u8);
    timer.set_timer(0, timer.get_mtime() + 10_000_000);
    // enable timer interrupts
    riscv::csr::mie::enable_m_mode_hardware_timer();

    Ok(())
}

pub fn init() -> Result<(), Error> {
    // init UART
    uart::Uart::new(memlayout::UART_BASE).init();

    // medeleg: delegate synchoronous exceptions except for ecall from HS-mode (bit 9)
    riscv::csr::medeleg::write(0xffffff ^ riscv::csr::medeleg::HYPERVISOR_ECALL );

    // mideleg: delegate all interruptions
    riscv::csr::mideleg::write(
        riscv::csr::mideleg::SEIP | riscv::csr::mideleg::STIP | riscv::csr::mideleg::SSIP,
    );
    
    // enable hypervisor extension
    let misa_state = riscv::csr::misa::read();
    riscv::csr::misa::write(misa_state | riscv::csr::misa::HV);
    assert_eq!(
        (riscv::csr::misa::read()) & riscv::csr::misa::HV,
        riscv::csr::misa::HV
    );

    // mtvec: set M-mode trap handler
    riscv::csr::mtvec::set(&(trap as unsafe extern "C" fn()));
    assert_eq!(
        riscv::csr::mtvec::read(),
        (trap as unsafe extern "C" fn()) as usize
    );

    // satp: disable paging
    riscv::csr::satp::write(0x0);

    // leave
    Ok(())
}

pub fn switch_to_hypervisor<T: util::jump::Target + Copy>(target: T) -> ! {
    riscv::csr::mstatus::set_mpp(riscv::csr::CpuMode::S);
    riscv::csr::mstatus::set_mpv(riscv::csr::VirtualzationMode::Host);
    riscv::csr::mepc::set(target);
    assert_eq!(
        riscv::csr::mepc::read(),
        target.convert_to_fn_address()
    );

    unsafe{
        asm!("	
        # Set up the PMP registers correctly
        li t4, 31
        csrw pmpcfg0, t4
        li t5, (1 << 55) - 1
        csrw pmpaddr0, t5
        ");
    }

    log::info!("Current mepc addr {:#x}", riscv::csr::mepc::read());
    riscv::instruction::mret();
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct TrapFrame {
    pub regs: [usize; 32],  // 0 - 255
    pub fregs: [usize; 32], // 256 - 511
    pub pc: usize,          // 512
}

#[no_mangle]
pub extern "C" fn rust_mtrap_handler(
    mepc: usize,           // a0
    mtval: usize,          // a1
    mcause: usize,         // a2
    mstatus: usize,        // a3
    frame: *mut TrapFrame, // a4
) -> usize {
    log::info!("trapped to M-mode!");
    let mcause = riscv::csr::mcause::read();
    log::info!("Machine trap cause {:#x}", mcause);
    let is_async = mcause >> 63 & 1 == 1;
    let cause_code = mcause & 0xfff;
    if is_async {
        match cause_code {
            7 => {
                riscv::csr::mip::set_stimer();
                let timer = clint::Clint::new(0x200_0000 as *mut u8);
                timer.set_timer(0, timer.get_mtime() + 10_000_000);
                //riscv::csr::mie::clear_m_mode_hardware_timer();
            }
            _ => {
                unimplemented!("Unknown M-mode interrupt id: {}", cause_code);
            }
        }
    } else {
        match cause_code {
            _ => {
                unimplemented!("Unknown M-mode Exception id: {}", cause_code);
            }
        }
    }
    mepc
}

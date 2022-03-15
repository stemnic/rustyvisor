global_asm!(include_str!("hypervisor.S"));

use crate::guest::Guest;
use crate::memlayout;
use crate::paging;
use crate::plic;
use crate::riscv;
use crate::riscv::gpr::Register;
use crate::uart;
use crate::virtio;
use crate::sbi;
use core::arch::asm;
use core::arch::global_asm;
use core::convert::TryFrom;
use core::fmt::Error;

use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec;

extern "C" {
    #[link_name = "hypervisor_entrypoint"]
    pub fn entrypoint();

    #[link_name = "trap_to_hypervisor"]
    pub fn trap();
}

#[no_mangle]
pub fn rust_hypervisor_entrypoint() -> ! {
    log::info!("hypervisor started");

    let k = Box::<u32>::new(100);
    println!("Boxed value = {}", *k);
    
    let sparkle_heart = vec![240, 159, 146, 150];
    let sparkle_heart = String::from_utf8(sparkle_heart).unwrap();
    println!("String = {}", sparkle_heart);

    if let Err(e) = init() {
        panic!("Failed to init rvvisor. {:?}", e)
    }
    log::info!("succeeded in initializing rvvisor");

    // TODO (enhnancement): multiplex here
    let guest_name = "guest01";
    log::info!("a new guest instance: {}", guest_name);
    log::info!("-> create metadata set");
    let mut guest = Guest::new(guest_name);
    log::info!("-> load a tiny kernel image");
    guest.load_from_disk();

    log::info!("switch to guest");
    switch_to_guest(&guest);
}

pub fn init() -> Result<(), Error> {
    // inti memory allocator
    paging::init();

    // init virtio
    virtio::init();

    // hedeleg: delegate some synchoronous exceptions
    riscv::csr::hedeleg::write(riscv::csr::hedeleg::INST_ADDR_MISALIGN 
                            | riscv::csr::hedeleg::BREAKPOINT 
                            | riscv::csr::hedeleg::ENV_CALL_FROM_U_MODE_OR_VU_MODE 
                            | riscv::csr::hedeleg::INST_PAGE_FAULT 
                            | riscv::csr::hedeleg::LOAD_PAGE_FAULT 
                            | riscv::csr::hedeleg::STORE_AMO_PAGE_FAULT);

    // hideleg: delegate all interrupts
    riscv::csr::hideleg::write(
        riscv::csr::hideleg::VSEIP | riscv::csr::hideleg::VSTIP | riscv::csr::hideleg::VSSIP,
    );

    // hvip: clear all interrupts first
    riscv::csr::hvip::write(0);

    // stvec: set handler
    riscv::csr::stvec::set(&(trap as unsafe extern "C" fn()));
    assert_eq!(
        riscv::csr::stvec::read(),
        (trap as unsafe extern "C" fn()) as usize
    );

    // allocate memory region for TrapFrame and set it sscratch
    let trap_frame = paging::alloc();
    riscv::csr::sscratch::write(trap_frame.address().to_usize());
    log::info!("sscratch: {:016x}", riscv::csr::sscratch::read());

    // enable interupts
    enable_interrupt();

    // TODO: hip and sip
    // TODO: hie and sie

    // leave
    Ok(())
}

fn enable_interrupt() {
    // TODO (enhancement): UART0

    // configure PLIC
    plic::enable_interrupt();

    // sie; enable external interrupt
    // TODO (enhancement): timer interrupt
    // TODO (enhancement): software interrupt
    let current_sie = riscv::csr::sie::read();
    riscv::csr::sie::write(current_sie | (riscv::csr::sie::SEIE as usize));

    // sstatus: enable global interrupt
    riscv::csr::sstatus::set_sie(true);
}

pub fn switch_to_guest(target: &Guest) -> ! {
    // hgatp: set page table for guest physical address translation
    riscv::csr::hgatp::set(&target.hgatp);
    riscv::instruction::hfence_gvma();
    assert_eq!(target.hgatp.to_usize(), riscv::csr::hgatp::read());

    // hstatus: handle SPV change the virtualization mode to 0 after sret
    riscv::csr::hstatus::set_spv(riscv::csr::VirtualzationMode::Guest);

    // sstatus: handle SPP to 1 to change the privilege level to S-Mode after sret
    riscv::csr::sstatus::set_spp(riscv::csr::CpuMode::S);

    // sepc: set the addr to jump
    riscv::csr::sepc::set(&target.sepc);

    // jump!
    riscv::instruction::sret();
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct TrapFrame {
    pub regs: [usize; 32],  // 0 - 255
    pub fregs: [usize; 32], // 256 - 511
    pub pc: usize,          // 512
}

fn show_trapinfo(
    sepc: usize,           // a0
    stval: usize,          // a1
    scause: usize,         // a2
    sstatus: usize,        // a3
    frame: *mut TrapFrame, // a4
){
    log::info!("<--------- trap --------->");
    log::info!("sepc: 0x{:016x}", sepc,);
    log::info!("stval: 0x{:016x}", stval,);
    log::info!("scause: 0x{:016x}", scause,);
    log::info!("sstatus: 0x{:016x}", sstatus,);

    log::info!("------- trapframe --------");
    let user_frame = unsafe{*frame.clone()};
    let mut i = 0;
    for reg in user_frame.regs {
        let reg_name = Register::try_from(i).unwrap();
        print!("{:<3} = 0x{:016x} ", reg_name, reg);
		if i % 4 == 3 {
			println!();
		} else {
			print!("| ")
		}
        i += 1;
    }
    log::info!("------- registers --------");
    riscv::gpr::dump();
    log::info!("---------  S csr ---------");
    riscv::csr::dump_s_csr();
    log::info!("---------  H csr ---------");
    riscv::csr::dump_h_csr();
    log::info!("--------- VS csr ---------");
    riscv::csr::dump_vs_csr();
    log::info!("-------- Prev Mode -------");
    let prev = riscv::csr::hstatus::previous_mode().unwrap();
    let mode_str = match prev {
        riscv::csr::PreviousMode::U_mode  => "User mode (U)",
        riscv::csr::PreviousMode::HS_mode => "Hypervisor mode (HS)",
        riscv::csr::PreviousMode::M_mode  => "Machine Mode (M)",
        riscv::csr::PreviousMode::VU_mode => "Virtual User Mode (VU)",
        riscv::csr::PreviousMode::VS_mode => "Virtual Supervisor Mode (VS)",
    };
    log::info!("Previous Mode before trap: {}", mode_str);
}

#[no_mangle]
pub extern "C" fn rust_strap_handler(
    sepc: usize,           // a0
    stval: usize,          // a1
    scause: usize,         // a2
    sstatus: usize,        // a3
    frame: *mut TrapFrame, // a4
) -> usize {
    log::debug!("<--------- trap --------->");
    log::debug!("sepc: 0x{:016x}", sepc,);
    log::debug!("stval: 0x{:016x}", stval,);
    log::debug!("scause: 0x{:016x}", scause,);
    log::debug!("sstatus: 0x{:016x}", sstatus,);

    let is_async = scause >> 63 & 1 == 1;
    let cause_code = scause & 0xfff;
    if is_async {
        match cause_code {
            // external interrupt
            9 => {
                if let Some(interrupt) = plic::get_claim() {
                    log::debug!("interrupt id: {}", interrupt);
                    match interrupt {
                        1..=8 => {
                            virtio::handle_interrupt(interrupt);
                        }
                        10 => {
                            uart::handle_interrupt();
                        }
                        _ => {
                            unimplemented!()
                        }
                    }
                    plic::complete(interrupt);
                } else {
                    panic!("invalid state")
                }
            }
            // timer interrupt & software interrrupt
            _ => {
                unimplemented!("Unknown interrupt id: {}", cause_code);
            }
        }
    } else {
        match cause_code {
            8 => {
                log::info!("environment call from U-mode / VU-mode at 0x{:016x}", sepc);
                // TODO: better handling
                loop {}
            }
            10 => {
                log::info!("environment call from VS-mode at 0x{:016x}", sepc);
                let user_frame = unsafe{*frame.clone()};
                match user_frame.regs[17] {
                    sbi::ecall::EXTENSION_TIMER => {

                    }
                    _ => {}
                }
                
                // TODO: better handling
                loop {}
            }
            21 => {
                log::info!("exception: load guest page fault at 0x{:016x}", sepc);
                show_trapinfo(sepc,stval,scause,sstatus,frame);
                // TODO (enhancement): demand paging
                loop {}
            }
            23 => {
                log::info!("exception: store/amo guest-page fault at 0x{:016x}", sepc);
                show_trapinfo(sepc,stval,scause,sstatus,frame);
                // TODO: better handling
                loop {}
            }
            _ => {
                show_trapinfo(sepc,stval,scause,sstatus,frame);
                unimplemented!("Unknown Exception id: {}", cause_code);
            }
        }
    }
    sepc
}

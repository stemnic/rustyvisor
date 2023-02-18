global_asm!(include_str!("kernel.S"));

use crate::memlayout;
use crate::paging;
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


    setup_vm();

    let call = sbi_call(EXTENSION_TIMER, 0x0, 0xdead, 0xbeef);
    println!("Sbi call error: {},  value: {}", call.error, call.value);
    assert_eq!(call.error, 0);
    
    println!("Testing timer");
    enable_timer_interrupts();

    loop {}
}

fn setup_vm() {
    paging::init();

    let root_page = paging::alloc();
    println!(
        "a page 0x{:016x} was allocated for a guest page address translation page table",
        root_page.address().to_usize()
    );
    let root_pt = paging::PageTable::from_page(root_page);

    let map_page_num = (unsafe{memlayout::elf_end()} - memlayout::DRAM_START)
    / (memlayout::PAGE_SIZE as usize)
    + 1;
    for i in 0..map_page_num {
        let vaddr = memlayout::DRAM_START + i * (memlayout::PAGE_SIZE as usize);
        let page = paging::Page::from_address(paging::PhysicalAddress::new(vaddr));
        root_pt.map(
            paging::VirtualAddress::new(vaddr),
            &page,
            (paging::PageTableEntryFlag::Read as u16)
                | (paging::PageTableEntryFlag::Write as u16)
                | (paging::PageTableEntryFlag::Execute as u16)
        )
    }

    let vaddr = memlayout::UART_BASE;
    let page = paging::Page::from_address(paging::PhysicalAddress::new(vaddr));
    root_pt.map(
        paging::VirtualAddress::new(vaddr),
        &page,
        (paging::PageTableEntryFlag::Read as u16)
            | (paging::PageTableEntryFlag::Write as u16)
            | (paging::PageTableEntryFlag::Execute as u16)
    );
    

    let map_page_num = (memlayout::USER_TEST_END - memlayout::USER_TEST_START)
        / (memlayout::PAGE_SIZE as usize)
        + 1;
    for i in 0..map_page_num {
        let vaddr = memlayout::USER_TEST_START + i * (memlayout::PAGE_SIZE as usize);
        let page = paging::Page::from_address(paging::PhysicalAddress::new(vaddr));
        root_pt.map(
            paging::VirtualAddress::new(vaddr),
            &page,
            (paging::PageTableEntryFlag::Read as u16)
                | (paging::PageTableEntryFlag::Write as u16)
                | (paging::PageTableEntryFlag::Execute as u16)
        )
    }

    let mut satp: usize = 0;
    satp |= (8 as usize) << 60;
    satp |= (0 as usize) << 44;
    satp |=  root_pt.page.address().to_ppn();

    println!("satp to be written: 0x{:016x}", satp);
    root_pt.print_page_allocations();

    unsafe{
        asm!(
            "csrw satp, {0}", in(reg) satp, options(nostack)
        );

        //asm!("sfence.vma");
    }
}

fn enable_timer_interrupts() {
    
    let mut sstatus:usize;
    unsafe{
        asm!(
            "csrr {0}, sstatus", out(reg) sstatus, options(nostack)
        )
    }
    let sie_mask = 1 << 1 as usize;
    let result: usize = sstatus | sie_mask;
    unsafe{
        asm!(
            "csrw sstatus, {0}", in(reg) result, options(nostack)
        )
    }


    let mut sie:usize;
    unsafe{
        asm!(
            "csrr {0}, sie", out(reg) sie, options(nostack)
        )
    }
    let sie_mask = 1 << 5 as usize;
    let result: usize = sie | sie_mask;
    unsafe{
        asm!(
            "csrw sie, {0}", in(reg) result, options(nostack)
        )
    }

}

fn clear_timers() {
    let mut sie:usize;
    unsafe{
        asm!(
            "csrr {0}, sie", out(reg) sie, options(nostack)
        )
    }
    let sie_mask = !(1 << 5) as usize;
    let result: usize = sie & sie_mask;
    unsafe{
        asm!(
            "csrw sie, {0}", in(reg) result, options(nostack)
        )
    }
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

    println!("trap set to: {:?}", &(trap as unsafe extern "C" fn()) );

    unsafe{
        asm!(
            "csrw stvec, {0}", in(reg) (trap as unsafe extern "C" fn() as usize)
        );
    }
    let mut stvec: usize;
    unsafe{
        asm!(
            "csrr {0}, stvec", out(reg) stvec
        );
    }

    println!("stvec is set to: 0x{:016x}", stvec);


    // leave
    Ok(())
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct TrapFrame {
    pub regs: [usize; 32],  // 0 - 255
    pub fregs: [usize; 32], // 256 - 511
    pub pc: usize,          // 512
}

#[no_mangle]
pub extern "C" fn rust_trap_handler(
    sepc: usize,           // a0
    stval: usize,          // a1
    scause: usize,         // a2
    sstatus: usize,        // a3
    frame: *mut TrapFrame, // a4
) -> usize {
    println!("<--------- trap --------->");
    println!("sepc: 0x{:016x}", sepc,);
    println!("stval: 0x{:016x}", stval,);
    println!("scause: 0x{:016x}", scause,);
    println!("sstatus: 0x{:016x}", sstatus,);
    let is_async = scause >> 63 & 1 == 1;
    let cause_code = scause & 0xfff;
    if is_async {
        match cause_code {
            5 => {
                //log::info!("M mode timer triggered");
                println!("vm timer interrupt triggered");
                //clear_timers();
                let call = sbi_call(EXTENSION_TIMER, 0x0, 0xdead, 0xbeef);
                println!("Sbi call error: {},  value: {}", call.error, call.value);
                assert_eq!(call.error, 0);
                
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
    sepc
}

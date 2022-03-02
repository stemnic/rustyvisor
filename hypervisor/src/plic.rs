use crate::memlayout;

pub fn enable_interrupt() {
    // TODO (enhancement): UART0

    // configure PLIC
    unsafe {
        let plic_base = memlayout::PLIC_BASE as *mut u32;
        plic_base
            .offset(memlayout::VIRTIO0_IRQ as isize)
            .write_volatile(1);
        plic_base
            .offset(memlayout::UART0_IRQ as isize)
            .write_volatile(1);
        plic_base
            .offset((memlayout::PLIC_HART1_M_INT_EN / 4)  as isize)
            .write_volatile((1 << memlayout::VIRTIO0_IRQ) | (1 << memlayout::UART0_IRQ));
        plic_base.offset((memlayout::PLIC_HART1_M_PRIORITY / 4) as isize).write_volatile(0);
    }
}

pub fn complete(interrupt: u32) {
    let plic_base = memlayout::PLIC_BASE as *mut u32;
    unsafe { plic_base.offset((memlayout::PLIC_HART1_M_CLAIM_COMPLETE / 4)  as isize ).write_volatile(interrupt) }
}

pub fn get_claim() -> Option<u32> {
    let plic_base = memlayout::PLIC_BASE as *mut u32;
    unsafe {
        let v = plic_base.offset((memlayout::PLIC_HART1_M_CLAIM_COMPLETE / 4) as isize ).read_volatile();
        if v == 0 {
            None
        } else {
            Some(v)
        }
    }
}

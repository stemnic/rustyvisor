use crate::timer::TimerHandle;
use rustsbi::{Forward, RustSBI};

#[derive(RustSBI)]
pub struct VmSBI {
    timer: TimerHandle,
    #[rustsbi(info)]
    forward: Forward,
}

impl VmSBI {
    #[inline]
    pub fn with_guest_number(guest_number: u64) -> Self {
        VmSBI {
            timer: TimerHandle::new(guest_number),
            forward: Forward,
        }
    }
}

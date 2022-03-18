const FUNCTION_TIMER_SET_TIMER: usize = 0x0;
use super::ecall::SbiRet;
#[inline]
#[cfg(target_pointer_width = "64")]
pub fn handle_ecall_timer_64(function: usize, param0: usize, guest_number: usize) -> SbiRet {
    match function {
        FUNCTION_TIMER_SET_TIMER => set_timer(param0, guest_number),
        _ => SbiRet::not_supported(),
    }
}

#[inline]
#[cfg(target_pointer_width = "32")]
pub fn handle_ecall_timer_32(function: usize, param0: usize, param1: usize, guest_number: usize) -> SbiRet {
    match function {
        FUNCTION_TIMER_SET_TIMER => set_timer(param0, param1, guest_number),
        _ => SbiRet::not_supported(),
    }
}

use crate::timer::VmTimers;

lazy_static::lazy_static! {
    pub static ref TIMER: spin::Mutex<VmTimers> = spin::Mutex::new(VmTimers::new());
}

#[cfg(target_pointer_width = "32")]
#[inline]
fn set_timer(arg0: usize, arg1: usize, guest_number: usize) -> SbiRet {
    let time_value = (arg0 as u64) + ((arg1 as u64) << 32);
    if set_timer_value(time_value, guest_number) {
        SbiRet::ok(0)
    } else {
        // should be probed with probe_extension
        SbiRet::not_supported()
    }
}

#[cfg(target_pointer_width = "64")]
#[inline]
fn set_timer(arg0: usize, guest_number: usize) -> SbiRet {
    let time_value = arg0 as u64;
    if set_timer_value(time_value, guest_number) {
        SbiRet::ok(0)
    } else {
        // should be probed with probe_extension
        SbiRet::not_supported()
    }
    
}

/// Timer programmer support

pub trait Timer: Send {
    /// Programs the clock for next event after `stime_value` time.
    ///
    /// `stime_value` is in absolute time. This function must clear the pending timer interrupt bit as well.
    ///
    /// If the supervisor wishes to clear the timer interrupt without scheduling the next timer event,
    /// it can either request a timer interrupt infinitely far into the future (i.e., (uint64_t)-1),
    /// or it can instead mask the timer interrupt by clearing `sie.STIE` CSR bit.
    fn set_timer(&mut self, stime_value: u64, guest_id: usize);
}

//static TIMER: OnceFatBox<dyn Timer + Sync + 'static> = OnceFatBox::new();
//static mut TIMER: VmTimers = VmTimers::new();

/* 
#[doc(hidden)] // use through a macro
pub fn init_timer<T: Timer + Sync + 'static>(timer: T) {
    let result = TIMER.set(Box::new(timer));
    if result.is_err() {
        panic!("load sbi module when already loaded")
    }
}
*/

/* 
#[inline]
pub fn probe_timer() -> bool {
    TIMER.get().is_some()
}
*/

#[inline]
pub fn set_timer_value(time_value: u64, guest_number: usize) -> bool {
    let mut timer = TIMER.lock();
    timer.set_timer(time_value, guest_number);
    
    true
}

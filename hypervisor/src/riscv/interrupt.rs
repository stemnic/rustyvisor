// Taken from the riscv rust package

//! Interrupts

// NOTE: Adapted from cortex-m/src/interrupt.rs
pub use bare_metal::{CriticalSection, Mutex};
use super::csr::mstatus;
use crate::m_mode_calls;

/// Disables all interrupts
#[inline]
pub unsafe fn disable() {
    mstatus::clear_mie();
    mstatus::clear_sie();
}

/// Enables all the interrupts
///
/// # Safety
///
/// - Do not call this function inside an `interrupt::free` critical section
#[inline]
pub unsafe fn enable() {
    mstatus::set_mie();
    mstatus::set_sie();
}

/// Execute closure `f` in an interrupt-free context.
///
/// This as also known as a "critical section".
pub fn free<F, R>(f: F) -> R
where
    F: FnOnce(&CriticalSection) -> R,
{

    // disable interrupts
    m_mode_calls::disable_interrupts();
    
    //unsafe {
    //    disable();
    //}

    let r = f(unsafe { &CriticalSection::new() });

    // If the interrupts were active before our `disable` call, then re-enable
    // them. Otherwise, keep them disabled

    m_mode_calls::enable_interrupts();

    //if mstatus::is_mie_set() {
    //    unsafe {
    //        enable();
    //    }
    //}

    r
}
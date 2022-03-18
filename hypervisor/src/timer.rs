use core::fmt::Error;
use crate::hypervisor::MAX_NUMBER_OF_GUESTS;
use crate::sbi::timer::Timer;

pub struct VmTimers {
    timers : [VmTimer; MAX_NUMBER_OF_GUESTS]
}

impl VmTimers {
    pub fn new() -> VmTimers {
        VmTimers{
            timers: [VmTimer::new() ; MAX_NUMBER_OF_GUESTS]
        }
    }
    pub fn tick_vm_timers(&mut self, amount: usize ){
        for mut timer in self.timers{
            timer.tick(amount as u64)
        }
    }
    pub fn check_timers(&self) -> [bool; MAX_NUMBER_OF_GUESTS] {
        let mut vm_timer_list = [false ; MAX_NUMBER_OF_GUESTS];
        let mut i = 0;

        while i < MAX_NUMBER_OF_GUESTS {
            let vmtimer = self.timers[i];
            if vmtimer.enabled {
                if vmtimer.mtime >= vmtimer.mtimecmp {
                    vm_timer_list[i] = true;
                }
            }
            i += 1;
        }
        return vm_timer_list
    }
}

#[derive(Copy, Clone)]
pub struct VmTimer {
    enabled: bool,
    mtime: u64,
    mtimecmp: u64
}

impl VmTimer {
    pub fn new() -> VmTimer {
        VmTimer{
            enabled: false,
            mtime: 0,
            mtimecmp: 0
        }
    }

    pub fn tick(&mut self, amount: u64){
        if self.enabled {
            self.mtime += amount;
        }
    }

    pub fn set_timer(&mut self, amount: u64){
        self.mtimecmp = amount;
    }
}

impl Timer for VmTimers {
    #[inline]
    fn set_timer(&mut self, time_value: u64, guest_id: usize) {
            self.timers[guest_id].set_timer(time_value);
    }
}
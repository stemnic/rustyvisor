use crate::memlayout;
use core::ptr;

// Based on fu540 CLINT

pub struct clint {
    msip : msip,
    mtimecmp: mtimecmp,
    mtime : mtime
}
#[repr(C, packed)]
struct mtime {
    mtime : u8
}
#[repr(C, packed)]
struct msip {
    h0_h1 : u8,
    h2_h3 : u8,
    h4    : u8,
}
#[repr(C, packed)]
struct mtimecmp {
    h0 : u8,
    h1 : u8,
    h2 : u8,
    h3 : u8,
    h4 : u8,
}

impl mtime {
    pub fn get_time(&self) -> u8 {
        self.mtime
    }
}

impl msip {
    pub fn h0_set(&mut self, value : usize) {
        self.h0_h1 = (self.h0_h1) | (value as u8 & 0xf)
    }
    pub fn h0_read(self) -> usize {
        (self.h0_h1 & 0xf) as usize
    }
    pub fn h1_set(&mut self, value : usize) {
        self.h0_h1 = (self.h0_h1) | (value as u8 & 0xf0)
    }
    pub fn h1_read(self) -> usize {
        ((self.h0_h1 & 0xf0) >> 4) as usize
    }
    pub fn h2_set(&mut self, value : usize) {
        self.h2_h3 = (self.h2_h3) | (value as u8 & 0xf)
    }
    pub fn h2_read(self) -> usize {
        (self.h2_h3 & 0xf) as usize
    }
    pub fn h3_set(&mut self, value : usize) {
        self.h2_h3 = (self.h2_h3) | (value as u8 & 0xf0)
    }
    pub fn h3_read(self) -> usize {
        ((self.h2_h3 & 0xf0) >> 4) as usize
    }
    pub fn h4_set(&mut self, value : usize) {
        self.h4 = (self.h2_h3) | (value as u8 & 0xf)
    }
    pub fn h4_read(self) -> usize {
        ((self.h4 & 0xf)) as usize
    }
}


impl clint {
    pub fn new() -> clint{
         clint{
            msip : msip{
                h0_h1 : 0xda,
                h2_h3 : 0xad,
                h4    : 0xbe,
            },
            mtimecmp : mtimecmp{
                h0 : 0xde,
                h1 : 0xad,
                h2 : 0xbe,
                h3 : 0xef,
                h4 : 0xf0,
            },
            mtime : mtime{
                mtime : 0xaa 
            }
        }
    }
    pub fn msip_addr(&self) -> usize {
        ptr::addr_of!(self.msip) as usize
    }
    pub fn mtimecmp_addr(&self) -> usize {
        ptr::addr_of!(self.mtimecmp) as usize
    }
    pub fn mtime_addr(&self) -> usize {
        ptr::addr_of!(self.mtime) as usize
    }
}
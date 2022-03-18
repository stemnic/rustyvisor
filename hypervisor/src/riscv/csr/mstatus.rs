use core::fmt::Error;
use super::{PreviousMode};

define_read!(0x300);
define_write!(0x300);

pub const MIE: usize = 0b1 << 3;
pub const SIE: usize = 0b1 << 1;

pub fn is_mie_set() -> bool {
    let mstatus = read();
    if ((mstatus >> 3) & 0b1) == 0b1 {
        true
    } else {
        false
    }
}

pub fn set_mie() {
    let mstatus = read();
    let mie_mask = MIE;
    write(mstatus | mie_mask);
}

pub fn clear_mie() {
    let mstatus = read();
    let mie_mask = !(MIE);
    write(mstatus & mie_mask);
}

pub fn set_sie() {
    let mstatus = read();
    let mie_mask = SIE;
    write(mstatus | mie_mask);
}

pub fn clear_sie() {
    let mstatus = read();
    let sie_mask = !(SIE);
    write(mstatus & sie_mask);
}

pub fn set_mpp(mode: crate::riscv::csr::CpuMode) {
    let mstatus = read();
    let mpp_mask = !(0b11 << 11 as usize);
    write((mstatus & mpp_mask) | ((mode as usize) << 11))
}

pub fn read_mpp() -> crate::riscv::csr::CpuMode {
    let mstatus = read();
    let mpv = (mstatus >> 11) & 0b11;
    if mpv == 0b00 {
        crate::riscv::csr::CpuMode::U
    } else if mpv == 0b01 {
        crate::riscv::csr::CpuMode::S
    } else {
        crate::riscv::csr::CpuMode::M
    }
}

pub fn set_mpv(mode: crate::riscv::csr::VirtualzationMode) {
    let mstatus = read();
    let mpv_mask = !(0b1 << 39 as usize);
    write((mstatus & mpv_mask) | ((mode as usize) << 39))
}

pub fn read_mpv() -> crate::riscv::csr::VirtualzationMode {
    let mstatus = read();
    if ((mstatus >> 39) & 0b1) == 0b0 {
        crate::riscv::csr::VirtualzationMode::Host
    } else {
        crate::riscv::csr::VirtualzationMode::Guest
    }
}

pub fn previous_mode() -> Result<PreviousMode, Error> {
    let mpv = read_mpv();
    let mpp = read_mpp();
    match mpv {
        super::VirtualzationMode::Host => {
            match mpp {
                super::CpuMode::M => Ok(PreviousMode::M_mode),
                super::CpuMode::S => Ok(PreviousMode::HS_mode),
                super::CpuMode::U => Ok(PreviousMode::U_mode),
            }
        },
        super::VirtualzationMode::Guest => {
            match mpp {
                super::CpuMode::M => Err(core::fmt::Error),
                super::CpuMode::S => Ok(PreviousMode::VS_mode),
                super::CpuMode::U => Ok(PreviousMode::VU_mode),
            }
        },
    }
}


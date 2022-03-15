define_read!(0x600);
define_write!(0x600);

use super::sstatus;
use super::PreviousMode;
use core::fmt::Error;

pub fn set_spv(mode: crate::riscv::csr::VirtualzationMode) {
    let hstatus = read();
    let spv_mask = !(0b1 << 7 as usize);
    write((hstatus & spv_mask) | ((mode as usize) << 7))
}

pub fn read_spv() -> crate::riscv::csr::VirtualzationMode {
    let hstatus = read();
    if ((hstatus >> 7) & 0b1) == 0b0 {
        crate::riscv::csr::VirtualzationMode::Host
    } else {
        crate::riscv::csr::VirtualzationMode::Guest
    }
}

pub fn previous_mode() -> Result<PreviousMode, Error> {
    let spv = read_spv();
    let spp = sstatus::read_spp();
    match spv {
        super::VirtualzationMode::Host => {
            match spp {
                super::CpuMode::M => Err(core::fmt::Error),
                super::CpuMode::S => Ok(PreviousMode::HS_mode),
                super::CpuMode::U => Ok(PreviousMode::U_mode),
            }
        },
        super::VirtualzationMode::Guest => {
            match spp {
                super::CpuMode::M => Err(core::fmt::Error),
                super::CpuMode::S => Ok(PreviousMode::VS_mode),
                super::CpuMode::U => Ok(PreviousMode::VU_mode),
            }
        },
    }
}
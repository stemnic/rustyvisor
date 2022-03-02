define_read!(0x602);
define_write!(0x602);

pub const INST_ADDR_MISALIGN: usize = 1 << 0;
pub const BREAKPOINT: usize = 1 << 3;
pub const ENV_CALL_FROM_U_MODE_OR_VU_MODE: usize = 1 << 8;
pub const INST_PAGE_FAULT: usize = 1 << 12; 
pub const LOAD_PAGE_FAULT: usize = 1 << 13; 
pub const STORE_AMO_PAGE_FAULT: usize = 1 << 15;
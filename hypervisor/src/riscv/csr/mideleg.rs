define_read!(0x303);
define_write!(0x303);

pub const SEIP: usize = 1 << 9;
pub const MTIP: usize = 1 << 7;
pub const STIP: usize = 1 << 5;
pub const MSIP: usize = 1 << 3;
pub const SSIP: usize = 1 << 1;

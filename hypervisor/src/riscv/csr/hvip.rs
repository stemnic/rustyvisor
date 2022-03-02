define_read!(0x645);
define_write!(0x645);

pub const VSSIP: usize = 1 << 2;
pub const VSTIP: usize = 1 << 6;
pub const VSEIP: usize = 1 << 10;

pub fn trigger_software_interrupt(){
    write( VSSIP | read() );
}

pub fn trigger_timing_interrupt(){
    write( VSTIP | read() );
}

pub fn trigger_external_interrupt(){
    write( VSEIP | read() );
}
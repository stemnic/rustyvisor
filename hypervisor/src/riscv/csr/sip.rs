define_read!(0x144);
define_write!(0x144);

pub const STIP: usize = 0b1 << 5;

pub fn clear_stimer() {
    let mip = read();
    write(mip & !(STIP))
}
define_read!(0x344);
define_write!(0x344);

const STIP: usize = 0b1 << 5;

pub fn set_stimer() {
    let mip = read();
    let stip_mask = !(STIP);
    write((mip & stip_mask) | STIP)
}

pub fn clear_stimer() {
    let mip = read();
    let stip_mask = !(STIP);
    write((mip & stip_mask) | ((0 as usize) << 5))
}
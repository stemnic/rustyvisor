define_read!(0x104);
define_write!(0x104);

pub const SEIE: usize = 1 << 9;

const STIE : usize = 0b1 << 5;

pub fn enable_hardware_timer() {
    let sie = read();
    let stie_mask = !(STIE);
    write((sie & stie_mask) | ((1 as usize) << 5))
}

pub fn clear_hardware_timer() {
    let sie = read();
    write(sie & !(STIE))
}

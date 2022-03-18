define_read!(0x304);
define_write!(0x304);

const MTIE : usize = 0b1 << 7;
const STIE : usize = 0b1 << 5;

pub fn enable_m_mode_hardware_timer() {
    let mie = read();
    let mtie_mask = !(MTIE);
    write((mie & mtie_mask) | ((1 as usize) << 7))
}

pub fn clear_m_mode_hardware_timer() {
    let mie = read();
    let mtie_mask = !(MTIE);
    write((mie & mtie_mask) | ((0 as usize) << 7))
}

pub fn enable_s_mode_hardware_timer() {
    let mie = read();
    let stie_mask = !(STIE);
    write((mie & stie_mask) | (STIE))
}

pub fn clear_s_mode_hardware_timer() {
    let mie = read();
    let stie_mask = !(STIE);
    write((mie & stie_mask) | ((0 as usize) << 5))
}


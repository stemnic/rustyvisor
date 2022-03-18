#[macro_use]
mod macros;

#[derive(PartialEq)]
pub enum CpuMode {
    M = 0b11,
    S = 0b01,
    U = 0b00,
}

pub enum VirtualzationMode {
    Host = 0,
    Guest = 1,
}

pub enum PreviousMode {
    U_mode,
    HS_mode,
    M_mode,
    VU_mode,
    VS_mode 
}

pub mod medeleg;
pub mod mepc;
pub mod mideleg;
pub mod mie;
pub mod mip;
pub mod misa;
pub mod mstatus;
pub mod mtvec;
pub mod mcause;
pub mod mhartid;

pub mod satp;
pub mod sepc;
pub mod sie;
pub mod sscratch;
pub mod sstatus;
pub mod stvec;
pub mod scounteren;
pub mod scause;
pub mod stval;
pub mod sip;

pub fn dump_s_csr() {
    let s_csr  = [
        satp::read(), 
        sepc::read(), 
        sie::read(), 
        sscratch::read(), 
        sstatus::read(), 
        stvec::read(), 
        scounteren::read(), 
        scause::read(), 
        stval::read(), 
        sip::read()];
    let s_csr_name    = [
        "satp", 
        "sepc", 
        "sie", 
        "sscratch", 
        "sstatus", 
        "stvec", 
        "scounteren", 
        "scause", 
        "stval", 
        "sip"];
    for i in 0..s_csr.len() {
		print!("{:<10} = 0x{:016x} ", s_csr_name[i], s_csr[i]);
		if i % 4 == 3 || i == s_csr.len()-1 {
			println!();
		} else {
			print!("| ")
		}
	}
}

pub mod hcontext;
pub mod hedeleg;
pub mod hcounteren;
pub mod hgatp;
pub mod hgeie;
pub mod hgeip;
pub mod hideleg;
pub mod hie;
pub mod hip;
pub mod hstatus;
pub mod htval;
pub mod hvip;
pub mod htimedelta;

pub fn dump_h_csr() {
    let h_csr  = [
        //hcontext::read(), // Causes fault when read
        hedeleg::read(), 
        hcounteren::read(), 
        hgatp::read(), 
        hgeie::read(), 
        hgeip::read(), 
        hideleg::read(), 
        hie::read(), 
        hip::read(), 
        hstatus::read(), 
        htval::read(), 
        hvip::read(), 
        htimedelta::read()];
    let h_csr_name    = [
        //"hcontext", 
        "hedeleg", 
        "hcounteren", 
        "hgatp", 
        "hgeie", 
        "hgeip", 
        "hideleg", 
        "hie", 
        "hip", 
        "hstatus", 
        "htval", 
        "hvip", 
        "htimedelta"];
    for i in 0..h_csr.len() {
		print!("{:<10} = 0x{:016x} ", h_csr_name[i], h_csr[i]);
		if i % 4 == 3 || i == h_csr.len()-1 {
			println!();
		} else {
			print!("| ")
		}
	}
}

pub mod vsatp;
pub mod vscause;
pub mod vsepc;
pub mod vsie;
pub mod vsip;
pub mod vsscratch;
pub mod vsstatus;
pub mod vstval;
pub mod vstvec;

pub fn dump_vs_csr(){
    let vs_csr  = [vsatp::read(), vscause::read(), vsepc::read(), vsie::read(), vsip::read(), vsscratch::read(), vsstatus::read(), vstval::read(), vstvec::read()];
    let vs_csr_name    = ["vsatp", "vscause", "vsepc", "vsie", "vsip", "vsscratch", "vsstatus", "vstval", "vstvec"];
    for i in 0..9 {
		print!("{:<10} = 0x{:016x} ", vs_csr_name[i], vs_csr[i]);
		if i % 4 == 3 || i == 9-1 {
			println!();
		} else {
			print!("| ")
		}
	}
}
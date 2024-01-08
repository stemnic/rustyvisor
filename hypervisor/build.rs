use std::{env, path::PathBuf};

fn main() {
    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let ld = &out.join("linker.ld");

    std::fs::write(ld, LINKER_SCRIPT).unwrap();

    println!("cargo:rustc-link-arg=-T{}", ld.display());
    println!("cargo:rustc-link-search={}", out.display());
}

const LINKER_SCRIPT: &[u8] = b"
OUTPUT_ARCH(riscv)

ENTRY(m_entrypoint)

SECTIONS
{
    . = 0x80000000; 
    .text.entrypoint : 
    {        
        PROVIDE(_elf_start = .);
        *(.text.entrypoint);
    }

    .text :
    {
        *(.text) *(.text.*);
    }

    .rodata :
    {
        *(.rdata .rodata. .rodata.*);
    }

    . = ALIGN(4096);
    .data :
    {
        *(.data .data.*);
    }

    _bss_start = .;
    .bss :
    {
        *(.bss .bss.*);
        PROVIDE(_elf_end = .);    
    }  
}";

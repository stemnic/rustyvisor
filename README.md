# rustyvisor

**NOTE: This project is still work in progress!**

## Requirements

This project relies on the following tools.

- [riscv/riscv-gnu-toolchain](https://github.com/riscv/riscv-gnu-toolchain)
- [QEMU with RISC-V Hypervisor Extension Emulation](https://github.com/kvm-riscv/qemu)
- Rust nightly

To run rustyvisor, you need to install them and configure your `PATH` first. 

## Usage

### Run rustyvisor with an example kernel

You can run the simple guest kernel, whose implementation is in `./guest` directory, as follows.

```sh
rustup target add riscv64gc-unknown-none-elf || true

# build hypervisor
cd hypervisor
cargo build
cd ..

# build guest
cd guest
cargo build
cd ..

# change path for qemu in hypervisor/.cargo/config
vim hypervisor/.cargo/config

# run hypervisor with guest
cd hypervisor
cargo run -- -drive file=../guest/target/riscv64gc-unknown-none-elf/debug/riscv-virt-guest,if=none,format=raw,id=x0 -device virtio-blk-device,drive=x0,bus=virtio-mmio-bus.0
```

### Run rustyvisor with your own kernel

Just change the path of the `--drive file=` to run a custom kernel with the hypervisor.

### NOTE: Debug rustyvisor with GDB

You can debug rustyvisor with gdb like this:

```sh
# in a shell ...
$ cargo run -- -drive file=../guest/target/riscv64gc-unknown-none-elf/debug/riscv-virt-guest,if=none,format=raw,id=x0 -device virtio-blk-device,drive=x0,bus=virtio-mmio-bus.0 -S -s # + additional opts

# in another shell ...
$ riscv64-unknown-elf-gdb hypervisor/target/riscv64gc-unknown-none-elf/debug/rustyvisor 

# add for the added init commands-x gdb_init_commands
...
(gdb) target remote :1234
(gdb) continue
```

## Features (to be supported)

rustyvisor currently supports the following features:

- :construction: Run a single VM upon rustyvisor
    - [x] load ELF image into the memory space of a VM
    - [x] jump to the kernel image loaded to a VM image while enabling guest physical address translation by `hgatp`
    - [x] run a tiny kernel that does not require any external hardware like disk devices
    - [ ] handle read/write requests for CSRs from a guest
    - [ ] handle SBI calls
- [ ] Run multiple VMs upon rustyvisor
    - [ ] switch CPU contexts among guests
    - [ ] schedule the guest in a fancy way
- [ ] Support multi-core host environment
- [ ] Support device virtualization
    - [ ] block device
    - [ ] network device
    - [ ] input device
    - [ ] display device

# Acknowledgments
rustyvisor is a continuation of the [rvvisor](https://github.com/lmt-swallow/rvvisor) originally created by Takashi Yoneuchi which laid the foundation for this project.
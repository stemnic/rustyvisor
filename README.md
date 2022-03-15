# rvvisor

**NOTE: This project is still work in progress!**

## Requirements

This project relies on the following tools.

- [riscv/riscv-gnu-toolchain](https://github.com/riscv/riscv-gnu-toolchain)
- [QEMU with RISC-V Hypervisor Extension Emulation](https://github.com/kvm-riscv/qemu)
- Rust nightly

To run rvvisor, you need to install them and configure your `PATH` first. 

## Usage

### Run rvvisor with an example kernel

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

### Run rvvisor with your own kernel

Just change the path of the `--drive file=` to run a custom kernel with the hypervisor.

### NOTE: Debug rvvisor with GDB

You can debug rvvisor with gdb like this:

```sh
# in a shell ...
$ cargo run -- -drive file=../guest/target/riscv64gc-unknown-none-elf/debug/riscv-virt-guest,if=none,format=raw,id=x0 -device virtio-blk-device,drive=x0,bus=virtio-mmio-bus.0 -S -s # + additional opts

# in another shell ...
$ riscv64-unknown-elf-gdb hypervisor/target/riscv64gc-unknown-none-elf/debug/rvvisor 

# add for the added init commands-x gdb_init_commands
...
(gdb) target remote :1234
(gdb) continue
```

## Features (to be supported)

rvvisor currently supports the following features:

- :construction: Run a single VM upon rvvisor
    - [x] load ELF image into the memory space of a VM
    - [x] jump to the kernel image loaded to a VM image whiel enabling guest physical address translation by `hgatp`
    - [x] run a tiny kernel which does not require any external hardwares like disk devices
    - [ ] handle read/write requests for CSRs from a guest
    - [ ] handle SBI calls
- [ ] Run multiple VMs upon rvvisor
    - [ ] switch CPU contexts among guests
    - [ ] schedule the guest in a fancy way
- [ ] Support multi-core host environment
- [ ] Support device virtualization
    - [ ] block device
    - [ ] network device
    - [ ] input device
    - [ ] display device
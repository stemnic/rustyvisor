TARGET 		:= riscv64gc-unknown-none-elf
MODE		:= debug

GUEST  		:= guest/target/$(TARGET)/$(MODE)/riscv-virt-guest

$(GUEST):
	cd guest && cargo build

run: $(GUEST)
	cd hypervisor && cargo run -- -drive file=../guest/target/riscv64gc-unknown-none-elf/debug/riscv-virt-guest,if=none,format=raw,id=x0 -device virtio-blk-device,drive=x0,bus=virtio-mmio-bus.0

clean:
	cd hypervisor && cargo clean 
	cd guest && cargo clean
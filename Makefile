LINKER := ld
MKGRUB := grub-mkrescue

iso := obj/tetanos.iso
kernel := obj/kernel.bin
assembly_source_files := $(wildcard src/bootloader/*.asm)
assembly_object_files := $(patsubst src/bootloader/%.asm, \
    obj/bootloader/%.o, $(assembly_source_files))

all: $(kernel) iso

kernel:
	RUST_TARGET_PATH=$(shell pwd)/targets xargo build --target x86_64_unknown-none

iso: $(iso)

obj/bootloader/%.o: src/bootloader/%.asm
	@mkdir -p obj/bootloader
	nasm -felf64 $< -o $@

run: $(iso)
	qemu-system-x86_64 -cdrom $(iso)

$(kernel): kernel $(assembly_object_files)
	$(LINKER) -n -T src/bootloader/linker.ld -o $(kernel) $(assembly_object_files)

$(iso): $(kernel)
	mkdir -p obj/isofiles/boot/grub
	cp $(kernel) obj/isofiles/boot/kernel.bin
	cp src/bootloader/grub.cfg  obj/isofiles/boot/grub
	$(MKGRUB) -o $(iso) obj/isofiles

clean:
	cargo clean
	rm -rf Cargo.lock
	rm -f **/*.o
	rm -rf obj/isofiles

doc:
	cargo doc --no-deps --open

format:
	cargo fmt

lint: format
	cargo clippy

test: lint
	cargo test --tests

bench:
	cargo bench --benches

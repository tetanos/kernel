LINKER := ld
MKGRUB := grub-mkrescue

iso := obj/tetanos.iso
kernel := obj/kernel.bin
libkernel := obj/libkernel.a
linker_script := src/bootloader/linker.ld
assembly_source_files := $(wildcard src/bootloader/*.asm)
assembly_object_files := $(patsubst src/bootloader/%.asm, \
    obj/bootloader/%.o, $(assembly_source_files))

all: iso
iso: $(iso)
kernel: $(libkernel)
run: $(iso)
	qemu-system-x86_64 -cdrom $(iso)

$(iso): $(kernel)
	mkdir -p obj/isofiles/boot/grub
	cp $(kernel) obj/isofiles/boot/kernel.bin
	cp src/bootloader/grub.cfg  obj/isofiles/boot/grub
	$(MKGRUB) -o $(iso) obj/isofiles

$(kernel): $(libkernel) $(assembly_object_files) $(linker_script)
	$(LINKER) -n -T $(linker_script) -o $(kernel) $(assembly_object_files) $(libkernel)

$(libkernel):
	RUST_TARGET_PATH=$(shell pwd)/targets xargo build --target x86_64_unknown-none
	cp target/x86_64_unknown-none/debug/libkernel.a $(libkernel)

obj/bootloader/%.o: src/bootloader/%.asm
	@mkdir -p obj/bootloader
	nasm -felf64 $< -o $@

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

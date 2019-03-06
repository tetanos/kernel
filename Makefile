ARCH?=x86_64
TARGET?=$(ARCH)_unknown-none

PROJECT_NAME?=tetanos
DEST_DIR?=target/$(TARGET)/disks
BUILD_DIR?=build
FILENAME?=$(PROJECT_NAME)-$(ARCH)
FILEPATH?=$(TARGET)/$(FILENAME)

NASM_INCLUDES?=-ibootloader/$(ARCH)/
KERNEL_NASM?= -D KERNEL=$(BUILD_DIR)/kernel.bin

pre-build: format
	mkdir -p $(DEST_DIR)
	mkdir -p $(BUILD_DIR)
	#cp src/lib.rs src/main.rs

post-build:
	rm -rf build #src/main.rs

bootloader: pre-build build/bootloader-$(ARCH).bin
	mv build/bootloader-$(ARCH).bin $(DEST_DIR)/bootloader-$(ARCH).bin
	$(MAKE) post-build

kernel: pre-build build/kernel-$(ARCH).o

build: pre-build build/$(FILENAME).a
	mv build/$(FILENAME).bin $(FILEPATH).bin
	make post-build

iso:
	echo todo

run:
	qemu-system-$(ARCH) -drive format=raw,file=$(FILEPATH).bin

run-bootloader: bootloader
	qemu-system-$(ARCH) -drive format=raw,file=$(DEST_DIR)/bootloader-$(ARCH).bin

clean:
	cargo clean
	rm -rf Cargo.lock
	$(MAKE) post-build

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

vbox: build/$(FILENAME).bin
	dd if=$< of=$(BUILD_DIR)/$(FILENAME).dd
	VboxManage convertdd $(BUILD_DIR)/$(FILENAME).dd $(BUILD_DIR)/$(FILENAME).vdi --format VDI --variant Fixed
	$(MAKE) post-build

build/bootloader-$(ARCH).bin:
	nasm -f bin -D ARCH_$(ARCH) $(NASM_INCLUDES) -o $@ bootloader/$(ARCH)/disk.asm

build/kernel-$(ARCH).a:
	cargo xbuild --target=targets/x86_64_unknown-none.json

build/$(FILENAME).bin: build/bootloader-$(ARCH).bin
	nasm -f bin -D ARCH_$(ARCH) $(KERNEL_NASM) $(NASM_INCLUDES) -o $@ bootloader/$(ARCH)/disk.asm


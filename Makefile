LINKER := ld
MKGRUB := grub-mkrescue

iso := obj/tetanos.iso
kernel := obj/kernel.bin
libkernel := obj/libkernel.a
linker_script := src/bootloader/linker.ld
source_files := $(wildcard src/**/*.rs) src/lib.rs
assembly_source_files := $(wildcard src/bootloader/*.asm)
assembly_object_files := $(patsubst src/bootloader/%.asm, \
    obj/bootloader/%.o, $(assembly_source_files))

##
##- Available targets:
##
help:		## This help dialog.
	@fgrep -h "##" $(MAKEFILE_LIST) | fgrep -v fgrep | sed -e 's/\\$$//' | sed -e 's/##//'

iso:		## Builds the iso file from the kernel bin and grub
iso: $(iso)
kernel:		## Links the libkernel and the bootloader
kernel: $(kernel)
libkernel:	## Builds the rust libkernel
libkernel: $(libkernel)
run:		## Runs the iso within qemu
run: $(iso)
	qemu-system-x86_64 -cdrom $(iso)

$(iso): $(kernel)
	mkdir -p obj/isofiles/boot/grub
	cp $(kernel) obj/isofiles/boot/kernel.bin
	cp src/bootloader/grub.cfg  obj/isofiles/boot/grub
	$(MKGRUB) -o $(iso) obj/isofiles

$(kernel): $(libkernel) $(assembly_object_files) $(linker_script)
	$(LINKER) -n -T $(linker_script) -o $(kernel) $(assembly_object_files) $(libkernel)

$(libkernel): $(source_files)
	RUST_TARGET_PATH=$(shell pwd)/targets xargo build --target x86_64_unknown-none
	cp target/x86_64_unknown-none/debug/libkernel.a $(libkernel)

obj/bootloader/%.o: src/bootloader/%.asm
	@mkdir -p obj/bootloader
	nasm -felf64 $< -o $@

docker:		## Runs "make iso" within our build docker image
	$(eval BUILDER := $(shell docker ps -a | grep 'tetanos-builder' | awk '{ print $$1; }' | head -n 1))
	docker build docker_ctx -t tetanos-builder
	if [ $(BUILDER) ]; then\
		docker start -ai $(BUILDER);\
	else\
		docker run --mount type=bind,source=$(shell pwd),target=/build -w /build -it tetanos-builder make iso;\
	fi

clean:		## Cleans the build folders
	$(eval BUILDER := $(shell docker ps -a | grep 'tetanos-builder' | awk '{ print $$1; }' | head -n 1))
	if [ $(BUILDER) ]; then\
		docker rm $(BUILDER);\
	fi
	docker rmi tetanos-builder
	cargo clean
	rm -rf obj/*
	docker r

doc:		## Builds and open the libkernel documentation
	cargo doc --no-deps --open

format:
	cargo fmt

lint: format
	cargo clippy

test: lint
	cargo test --tests

bench:
	cargo bench --benches

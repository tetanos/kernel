AS := nasm
AS_FLAGS := -fbin

all: run

build:
	$(AS) $(AS_FLAGS) -o boot boot.s
	$(AS) $(AS_FLAGS) -o init init.s
	cat boot init > tmp

run: build
	qemu-system-x86_64 -drive format=raw,file=tmp

debug: build
	qemu-system-x86_64 -drive format=raw,file=tmp -S -s


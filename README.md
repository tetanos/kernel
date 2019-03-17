# TetanOS
An attempt at building an operating system, make sure you're vaxxed.

The kernel is built in a docker container. (https://github.com/tetanos/builder)

## Build deps

- make
- nasm
- ld
- grub-mkrescue
  - xorriso
- qemu (optional)

## Rust deps

	curl https://sh.rustup.rs -sSf | sh
	rustup default nightly
	rustup component add rust-src
	cargo install xargo

## Build

	make iso
	
## Build from Docker

	make docker

## Run inside qemu

	make run

## Flash to a usb key

	dd if=obj/tetanos.iso of=/dev/diskX && sync

## License

TetanOS is MIT licensed.


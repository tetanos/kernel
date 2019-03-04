# TetanOS
An attempt at building an operating system, make sure you're vaxxed.

## Build and Run

Build the bootable binary and launch it in qemu

```sh
cargo make run
```

## Flash to a usb key

	cargo make build
	dd if=target/x86_64/debug/bootimage-tetan_os.bin of=/dev/diskX && sync

## Dev prereq

- build-essentials or equivalent
- qemu

## Setup dev env

	curl https://sh.rustup.rs -sSf | sh
	git clone https://github.com/afrigon/TetanOS
	cd TetanOS
	cargo build

## License

TetanOS is MIT licensed.


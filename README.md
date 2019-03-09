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
- binutils
- grub-mkrescue

## OSX specifics

install a cross-compiled binutils for the linker

	curl -O http://ftp.gnu.org/gnu/binutils/binutils-2.24.tar.gz
	tar -xvf binutils-2.24.tar.gz
	cd binutils-2.24
	./configure --target=x86_64-elf --prefix="$HOME/opt/cross" \
		--disable-nls --disable-werror \
		--disable-gdb --disable-libdecnumber --disable-readline --disable-sim
	make
	
make sure `$HOME/opt/cross` is in your PATH

and that the make `LINKER` variable is set to `x86_64-elf-ld`

## Setup dev env

	curl https://sh.rustup.rs -sSf | sh
	git clone https://github.com/afrigon/TetanOS
	cd TetanOS
	cargo build

## License

TetanOS is MIT licensed.


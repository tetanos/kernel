# TetanOS
An attempt at building an operating system, make sure you're vaccinated.

## Build dependencies

- make
- nasm
- ld
- grub-mkrescue
- xorriso

## Rust dependencies

```sh
curl https://sh.rustup.rs -sSf | sh
rustup component add --toolchain nightly rust-src
cargo install xargo
```

## Build

```sh
make iso
```

## Build from Docker

The docker image used is available [here](https://github.com/tetanos/builder).

```sh
make docker
```

## Run with qemu

`qemu-system-x86-64` is required to run this command.

```sh
make run
```

## Flash a usb drive

Be careful with this command, it will format your usb drive.

```sh
dd if=obj/tetanos.iso of=/dev/diskX && sync
```

## Note for OSX dev

GNU ld doesn't work on OSX, so we cross compile it for elf target

```sh
mkdir -p ~/src/ && cd ~/src/
wget https://ftp.gnu.org/gnu/binutils/binutils-2.24.tar.gz
tar -xvzf binutils-2.24.tar.gz && cd binutils-2.24
./configure --target=x86_64-elf --prefix="$HOME/opt/cross" \
	--disable-nls --disable-werror \
	--disable-gdb --disable-libdecnumber --disable-readline --disable-sim
make
make install
```

then set LINKER=~/cross/bin/x86_64-elf-ld when calling make (required for `kernel` target)

`make kernel LINKER=~/cross/bin/x86_64-elf-ld`

## License

TetanOS is MIT licensed.

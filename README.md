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

## License

TetanOS is MIT licensed.


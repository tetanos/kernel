# TetanOS
An attempt at building an operating system, make sure you're vaccinated.

## Build dependencies

- [mise](https://mise.jdx.dev) — pins the Rust toolchain and runs the build tasks
- nasm
- GNU ld
- grub-mkrescue
- xorriso

```sh
mise install
```

## Build

```sh
mise run build
```

The bootable image is written to `obj/tetanos.iso`.

## Build from Docker

The docker image used is available [here](https://github.com/tetanos/builder).

```sh
docker run --rm -v "$PWD":/build tetanos/builder
```

## Run with qemu

`qemu-system-x86_64` is required to run this command.

```sh
mise run qemu
```

## Flash a usb drive

Be careful with this command, it will format your usb drive.

```sh
dd if=obj/tetanos.iso of=/dev/diskX && sync
```

## Note for OSX dev

GNU ld doesn't produce the ELF kernel binary on OSX, so build from Docker
instead.

## License

TetanOS is MIT licensed.

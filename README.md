# TetanOS
An attempt at building an operating system, make sure you're vaxxed.

## Build


Make sure bootimage is installed, it compile the bootloader and prepend it to our kernel

```sh
cargo install bootimage
```


Build the bootable binary

```sh
bootimage build
```


Run the binary using qemu

```sh
qemu-system-x86_64 -drive format=raw,file=target/x86_64/debug/bootimage-tetan_os.bin
```


## License

TetanOS is MIT licensed.


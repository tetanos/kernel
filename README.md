# TetanOS
An attempt at building an operating system, make sure you're vaxxed.

## Build and Run

Build the bootable binary and launch it in qemu

```sh
cargo make run
```

## Flash to a usb key

`dd if=target/x86_64/debug/bootimage-tetan_os.bin of=/dev/diskX && sync`

## License

TetanOS is MIT licensed.


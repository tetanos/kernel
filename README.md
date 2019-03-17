# TetanOS
An attempt at building an operating system, make sure you're vaxxed.

The kernel is built in a docker container.

See https://github.com/tetanos/builder

## Build and Run

Build the bootable binary and launch it in qemu

	make run

## Flash to a usb key

	make iso
	dd if=obj/tetanos.iso of=/dev/diskX && sync

## License

TetanOS is MIT licensed.


# TetanOS
An attempt at building an operating system, make sure you're vaxxed.

The kernel is built in a docker container.

See https://github.com/tetanos/builder

## Build from docker

	make docker

## Build natively

	make iso

## Run inside qemu

	make run

## Flash to a usb key

	dd if=obj/tetanos.iso of=/dev/diskX && sync

## License

TetanOS is MIT licensed.


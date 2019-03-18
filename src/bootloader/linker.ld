ENTRY(loader_start)

SECTIONS {
    . = 1M;

    .boot :
    {
	KEEP(*(.multiboot_header))
    }

    .text :
    {
        *(.text)
    }

    .bss :
    {
        *(.bss)
    }

    .rodata :
    {
        *(.rodata)
    }
}

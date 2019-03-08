%define GRUB2_MAGIC 0xe85250d6

SECTION .multiboot_header
header_start:
    dd GRUB2_MAGIC
    dd 0
    dd header_end - header_start
    dd 0x100000000 - (GRUB2_MAGIC + (header_end - header_start))

    dw 0
    dw 0
    dd 8
header_end:


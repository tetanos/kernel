EXTERN kernel_main
GLOBAL start

USE32

SECTION .text
start:
    mov dword [0xb8000], 0x2f4b2f4f

    hlt



vga_pointer: db 0xb8000

str_title: db "TetanOS Loader v1",0

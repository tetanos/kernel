EXTERN kernel_main
GLOBAL start

USE32

SECTION .text
start:
    mov dword [0xb8000], 0x2f4b2f4f
    hlt


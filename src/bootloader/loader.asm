global loader_start

section .text
bits 32

%define MULTIBOOT_MAGIC 0x36d76289

; grub entry point
loader_start:
    mov esp, stack.top
	call multiboot.check              ; eax should contain the multiboot magic
	call cpuid.check
	call long_mode.check

    call memory.init_page_tables
    call memory.init_paging
    call memory.init_segmentation

    mov esi, error_multiboot
    call error
	jmp gdt.code:long_mode.enable

error:
    mov edx, esi
    mov esi, error_string
    call vga.set_style_error
    call vga.print
    mov esi, edx
    call vga.println
    hlt

multiboot:
.check:
	cmp eax, MULTIBOOT_MAGIC
	jne multiboot.error
	ret
.error:
	mov esi, error_multiboot
	jmp error

%include "vga.asm"
%include "cpuid.asm"
%include "memory.asm"
%include "long.asm"

section .rodata
error_string:
    db "An error occured, ",0
error_multiboot:
    db "could not validate the multiboot magic number",0

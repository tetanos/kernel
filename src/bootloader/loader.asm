global loader_start

section .text
bits 32

%define MULTIBOOT_MAGIC 0x36d76289

; grub entry point
loader_start:
    mov esp, stack.top

    push eax                            ; eax should contain the multiboot magic
    
    call vga.clear

    call vga.print_crlf
    mov esi, string_title
    call vga.println

    mov esi, string_init
    call log.check
    call log.check_ok

    mov esi, string_multiboot           ; multiboot check
    call log.check
    pop eax
	call multiboot.check              
    call log.check_ok

    mov esi, string_cpuid               ; cpuid check
    call log.check
	call cpuid.check
    call log.check_ok

    mov esi, string_long_mode           ; long mode check
    call log.check
	call long_mode.check
    call log.check_ok

    call vga.print_crlf

    call memory.init

    hlt
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
    call log.check_error
	mov esi, error_string_multiboot
	jmp error

%include "vga.asm"
%include "cpuid.asm"
%include "memory.asm"
%include "long.asm"

section .rodata
error_string:
    db "An error occured, ",0
error_string_multiboot:
    db "could not validate the multiboot magic number",0
string_title:
    db "                           +------------------------+",10,13
    db "                           |  TetanOS Loader, v0.1  |",10,13
    db "                           +------------------------+",10,13,0
string_init:
    db "vaccinating components",0
string_multiboot:
    db "checking multiboot magic",0

global loader_start

section .text
bits 32

; grub entry point
loader_start:
    mov esp, stack.top
	call multiboot.check
	call cpuid.check
	call long_mode.check

    call memory.init_page_tables
    call memory.init_paging
    call memory.init_segmentation

	jmp gdt.code:long_mode.enable

error:
;    mov eax, esi
;    mov esi, error_string
;    call vga.print
;    mov esi, eax
;    call vga.println
    mov dword [0xb8000], 0x4f524f45
	mov dword [0xb8004], 0x4f3a4f52
	mov dword [0xb8008], 0x4f204f20
	mov byte  [0xb800a], al
    hlt

multiboot:
.check:
	cmp eax, [multiboot_magic]
	jne multiboot.error
	ret

.error:
	mov al, "0"
	jmp error

%include "cpuid.asm"
%include "memory.asm"
%include "long.asm"

section .rodata
multiboot_magic:
    dd 0x36d76289
error_string:
    db "An error occured, ",0
error_multiboot:
    db "could not validate the multiboot magic number",0

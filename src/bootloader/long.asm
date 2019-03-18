extern kernel_start

section .text
bits 64

long_mode:
.check:
	;; test if extended processor info in available
	mov eax, 0x80000000	; implicit argument for cpuid
	cpuid			; get highest supported argument
	cmp eax, 0x80000001	; it needs to be at least 0x80000001
	jb long_mode.error	; if it's less, the CPU is too old for long mode

	;; use extended info to test if long mode is available
	mov eax, 0x80000001	; argument for extended processor info
	cpuid			; returns various feature bits in ecx and edx
	test edx, 1 << 29	; test if the LM-bit is set in the D-register
	jz long_mode.error	; If it's not set, there is no long mode
	ret

.error:
	mov al, "2"
	jmp error

; jump to the rust kernal enabling 64 bits long mode
.enable:
	mov ax, 0               ; clear segment registers
	mov ss, ax
	mov ds, ax
	mov es, ax
	mov fs, ax
	mov gs, ax
    
	call kernel_start       ; jump to the rust kernel entry point
    hlt

section .rodata
error_long_mode:
    db "long mode is not available on this system",0

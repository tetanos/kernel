extern kernel_start

section .text
bits 64

; https://en.wikipedia.org/wiki/CPUID
%define CPUID_LARGEST_EXTENDED_FUNCTION 0x80000000
%define CPUID_EXTENDED_FEATURES 0x80000001
%define CPUID_FEATURE_64BITS 1 << 29

long_mode:
; check if the system supports long mode
; CLOBBER
;   eax, edx
.check:
	mov eax, CPUID_LARGEST_EXTENDED_FUNCTION
	cpuid
	cmp eax, CPUID_EXTENDED_FEATURES            ; make sure cpuid supports extended features
	jb long_mode.error

	mov eax, CPUID_EXTENDED_FEATURES            ; get infos about extended features
	cpuid
	test edx, CPUID_FEATURE_64BITS              ; test if system support long mode
	jz long_mode.error
	ret

.error:
	mov esi, error_string_long_mode
	jmp error

; jump to the rust kernal enabling 64 bits long mode
; CLOBBER
;   ax
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
error_string_long_mode:
    db "long mode is not available on this system",0

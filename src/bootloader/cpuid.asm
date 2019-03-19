section .text
bits 32

cpuid:
; check if the system supports cpuid using bit 21 of eflags
; CLOBBER
;   eax, ecx
.check:
	pushfd                  ; load flags into eax
	pop eax

	mov ecx, eax            ; create a copy to restore

	xor eax, 1 << 21        ; flip bit 21

	push eax                ; push eax to flags and back to eax
	popfd
	pushfd
	pop eax

	push ecx                ; restore old flags
	popfd

	cmp eax, ecx            ; if the flags are different, bit 21 was flipped, the check was succesful
	je cpuid.error
	ret

.error:
	mov esi, error_string_cpuid
	jmp error

section .rodata
error_string_cpuid:
    db "cpuid is not available on this system",0

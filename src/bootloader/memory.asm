section .text:
bits 32

memory:
.init:
    call memory.init_page_tables
    call memory.init_paging
    call memory.init_segmentation
    ret

; initialize page tables linking them together
; CLOBBER
;   eax, ecx, esi
.init_page_tables:
    mov esi, string_page_tables
    call log.check

	mov ecx, page_tables.p3                 ; p4[0] points to p3
	or ecx, 0b11		                    ; mark as present and writable
	mov [page_tables.p4], ecx

	mov ecx, page_tables.p2                 ; p3[0] points to p2
	or ecx, 0b11		                    ; mark as present and writable
	mov [page_tables.p3], ecx

	mov ecx, 0
; map p2 entries to 2mb huge pages
; CLOBBER
;  eax, ecx 
.init_p2_loop:
	mov eax, 0x200000                       ; 2mb
	mul ecx
	or eax, 0b10000011	                    ; mark as present, writable and huge
	mov [page_tables.p2 + ecx * 8], eax     ; p2[ecx] points to 2mb page

	inc ecx
	cmp ecx, 512
	jne .init_p2_loop

    call log.check_ok
	ret

;
; CLOBBER
;  eax, ecx, esi
.init_paging:
    mov esi, string_paging
    call log.check

	mov eax, page_tables.p4                 ; cr3 = *p4
	mov cr3, eax
    
    mov ecx, cr4                            ; enable physical address extensions in cr4
    or ecx, 1 << 5
    mov cr4, ecx

	mov ecx, 0xC0000080                     ; enable long mode in the model specific register 0xC0000080
	rdmsr
	or eax, 1 << 8
	wrmsr

	mov eax, cr0                            ; enable paging in cr0
	or eax, 1 << 31
	mov cr0, eax

    call log.check_ok
	ret

; load a basic global descriptor table
.init_segmentation:
    mov esi, string_segmentation
    call log.check

    lgdt [gdt.pointer]

    call log.check_ok
    ret

section .bss
align 4096

page_tables:
.p4:
	resb 4096
.p3:
	resb 4096
.p2:
	resb 4096
stack:
.bottom:
	resb 64
.top:

section .rodata
bits 64

gdt:
	dq 0					                    ; zero entry
.code: equ $ - gdt
	dq (1<<43) | (1<<44) | (1<<47) | (1<<53)    ; code segment
.pointer:
	dw $ - gdt - 1
	dq gdt

string_page_tables:
    db "loading page tables",0
string_paging:
    db "initializing paging",0
string_segmentation:
    db "segmenting memory",0

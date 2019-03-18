section .text:
bits 32

memory:
.init_page_tables:
	;; map first P4 entry to P3 table
	mov eax, page_tables.p3
	or eax, 0b11		; present + writable
	mov [page_tables.p4], eax

	;; map first P3 entry to P2 table
	mov eax, page_tables.p2
	or eax, 0b11		; present + writable
	mov [page_tables.p3], eax

	;; map each P2 entry to a huge 2MiB page
	mov ecx, 0		; counter variable

.map_p2:
	;; map ecx-th P2 entry to a huge page that starts at address 2MiB*ecx
	mov eax, 0x200000	      ; 2MiB
	mul ecx			      ; start address of ecx-th page
	or eax, 0b10000011	      ; present + writable + huge
	mov [page_tables.p2 + ecx * 8], eax ; map ecx-th entry
	inc ecx			      ; increase counter
	cmp ecx, 512		      ; if counter == 512, the whole P2 table is mapped
	jne .map_p2	      ; else map the next entry
	ret

.init_paging:
	;; load P4 to cr3 register (cpu uses this to access the P4 table)
	mov eax, page_tables.p4
	mov cr3, eax
	;; enable PAE-flag in cr4 (Physical Address Extension)
	mov eax, cr4
	or eax, 1 << 5
	mov cr4, eax
	;; set the long mode bit in the EFER MSR (model specific register)
	mov ecx, 0xC0000080
	rdmsr
	or eax, 1 << 8
	wrmsr
	;; enable paging in the cr0 register
	mov eax, cr0
	or eax, 1 << 31
	mov cr0, eax
	ret

.init_segmentation:
    lgdt [gdt.pointer]
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

global long_mode_start
extern kernel_start
bits 64

long_mode_start:
	mov ax, 0             ; clear segment registers
	mov ss, ax
	mov ds, ax
	mov es, ax
	mov fs, ax
	mov gs, ax
    
	call kernel_start      ; jump to the rust kernel entry point
    hlt

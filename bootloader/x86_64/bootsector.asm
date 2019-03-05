ORG 0x7c00
SECTION .text
USE16

; bootsector entry - stage 1
bootsector:
    mov [disk_id], dl   ; save the drive id (passed by the bios)

    xor ax, ax          ; clear the segment registers
    mov ds, ax
    mov es, ax
    mov ss, ax

    mov sp, 0x7c00      ; initialize the stack

    call print_crlf
    mov si, str_title
    call println

    jmp branch_error


branch_success:
    jmp stage2


branch_error:
    mov si, str_error
    call print
.halt:                  ; put the system to sleep forever
    cli
    hlt
    jmp .halt


; print a string and a crlf
; IN
;   si: points to the address of the string
; CLOBBER
;   si, ax
println:
    call print
    call print_crlf
    ret


; does a carriage return and a line feed
; CLOBBER
;   al
print_crlf:
    mov al, 0xd
    call print_char
    mov al, 0xa
    call print_char
    ret


; print a string to the teletype output
; IN
;   si: points to the address of the string
; CLOBBER
;   si, ax
print:
    pushf               ;   push flags
    cld                 ;   clear the direction flag (makes lodsb increment si)
.loop:
    lodsb               ;   load byte from ds:si (data section:source index register) into al
    test al, al         ;   set zf to 1 if al & al = 0
    jz .end             ;   jmp to end if zf is on (al is a null byte)
    
    call print_char
    jmp .loop
.end:
    popf                ; pop flags
    ret


; print a character to the teletype output
; IN
;   al: character to print
print_char:
    pusha               ;   push all general purpose registers
    mov ah, 0xe         ;   video - teletype output function
    mov bx, 0xf         ;   page 0, white text           
    int 0x10            ;   video bios call
    popa                ;   pop all general purpose registers
    ret


str_title: db "TetanOS Bootloader - Stage One",0
str_error: db "Could not enter stage two. ",13,10,"Halting...",0

disk_id: db 0

disk_access_paquet:
          db 0x10       ; paquet size
          db 0          ; unused
.count:   dw 0          ; number of block read/written
.buffer:  dw 0          ; offset to memory buffer
.segment: dw 0          ; memory page
.address: dq 0          ; logical block address to read

times 510-($-$$) db 0   ; padding to offset 510
dw 0xaa55               ; magic bootsector number 0xaa55


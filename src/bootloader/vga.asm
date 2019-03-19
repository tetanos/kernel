section .text
bits 32

%define VGA_BUFFER_ADDRESS 0xb8000
%define VGA_BUFFER_WIDTH 80
%define VGA_BUFFER_HEIGHT 25
%define VGA_STYLE_WHITE 0xf
%define VGA_STYLE_RED 0xc

vga:
; print a string and the crlf sequence
; IN
;   esi: pointer to a null-terminated string
; CLOBBER
;   ax, edi, edx
.println:
    call vga.print
    mov al, 13
    call vga.print_char
    mov al, 10
    call vga.print_char
    ret

; print a string
; IN
;   esi: pointer to a null-terminated string
; CLOBBER
;   ax, edi, edx
.print:
    pushf
    cld
.print_loop:
    lodsb
    test al, al
    jz vga.print_end

    call vga.print_char
    jmp vga.print_loop
.print_end:
    popf
    ret


; print a character
; IN
;   al: character to print
; CLOBBER
;   ax, edi, edx
.print_char:
    cmp al, 13
    je vga.print_cr

    cmp al, 10
    je vga.print_lf

    mov ah, VGA_STYLE_WHITE

    xor ecx, ecx
    imul cx, [vga_cursor.y], VGA_BUFFER_WIDTH
    add cx, [vga_cursor.x]
    shl cx, 1                                  ; edi *= 2 to take style bytes into account

    mov word [ecx + VGA_BUFFER_ADDRESS], ax     ; actual write to the VGA text buffer

    inc word [vga_cursor.x]
    cmp word [vga_cursor.x], VGA_BUFFER_WIDTH   ; if the cursor is going offscreen to the right,
    jae vga.wrap_line                           ; wrap to the next line
    ret
.print_cr:
    mov word [vga_cursor.x], 0
    ret
.print_lf:
    cmp word [vga_cursor.y], VGA_BUFFER_HEIGHT  ; if the cursor is going offscreen to the bottom,
    jae .scroll_up                              ; scroll the entire buffer up by one row

    inc word [vga_cursor.y]
    ret
.wrap_line:
    call vga.print_cr
    call vga.print_lf
    ret
.scroll_up:
    
    ret

section .data
vga_cursor:
.x:
    dw 0
.y:
    dw 0

section .text
bits 32

%define VGA_BUFFER_ADDRESS 0xb8000
%define VGA_BUFFER_WIDTH 80
%define VGA_BUFFER_HEIGHT 25
%define VGA_STYLE_WHITE 0xf
%define VGA_STYLE_RED 0xc

vga:
; print a string and a newline
; IN
;   esi: points at zero-terminated String
.println:
;    push eax
;    push ebx
;    push ecx
;    push edx
;
;    call vga_print
;
;    ; newline
;    mov edx, 0
;    mov eax, vga_position
;    mov ecx, 80 * 2
;    div ecx
;    add eax, 1
;    mul ecx
;    mov vga_position, eax
;
;    pop edx
;    pop ecx
;    pop ebx
;    pop eax
;
    ret

; print a string
; IN
;   esi: points at zero-terminated String
; CLOBBER
;   ah, ebx
.print:
    pushf
    cld
.loop:
    lodsb
    test al, al
    jz .end

    call vga.print_char
    jmp .loop
.end:
    popf
    ret


; print a character
; IN
;   al: character to print
; CLOBBER
;   ah, ebx
.print_char:
    cmp al, 13
    je .print_cr

    cmp al, 10
    je .print_lf

    mov ah, VGA_STYLE_WHITE

    mov edi, [vga_cursor.x]
    shl edi, 1                              ; take style bytes into account
    mov word [edi + VGA_BUFFER_ADDRESS], ax

    inc byte [vga_cursor.x]
    ret
.print_cr:
    mov byte [vga_cursor.x], 0
    ret
.print_lf:
    cmp byte [vga_cursor.y], VGA_BUFFER_HEIGHT
    jae .scroll_up

    inc byte [vga_cursor.y]
    ret
.wrap_line:
    call .print_cr
    call .print_lf
    ret
.scroll_up:
    
    ret

section .data
vga_cursor:
.x:
    db 0
.y:
    db 0

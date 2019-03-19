section .text
bits 32

%define VGA_BUFFER_ADDRESS 0xb8000
%define VGA_BUFFER_WIDTH 80
%define VGA_BUFFER_HEIGHT 25
%define VGA_BUFFER_ROW_SIZE VGA_BUFFER_WIDTH * 2

%define VGA_STYLE_GRAY 0x7
%define VGA_STYLE_BLUE 0x9
%define VGA_STYLE_GREEN 0xa
%define VGA_STYLE_RED 0xc
%define VGA_STYLE_YELLOW 0xe
%define VGA_STYLE_WHITE 0xf

log:
; log a string with normal style
; IN
;   esi: pointer to a null-terminated string
; CLOBBER
;   eax, ecx
.info:
    call vga.set_style_info
    call vga.println
    call vga.set_style_normal
    ret

; log a string with ok style
; IN
;   esi: pointer to a null-terminated string
; CLOBBER
;   eax, ecx
.ok:
    call vga.set_style_ok
    call vga.println
    call vga.set_style_normal
    ret

; log a string with success style
; IN
;   esi: pointer to a null-terminated string
; CLOBBER
;   eax, ecx
.success:
    call vga.set_style_success
    call vga.println
    call vga.set_style_normal
    ret

; log a string with warning style
; IN
;   esi: pointer to a null-terminated string
; CLOBBER
;   eax, ecx
.warning:
    call vga.set_style_warning
    call vga.println
    call vga.set_style_normal
    ret

; log a string with error style
; IN
;   esi: pointer to a null-terminated string
; CLOBBER
;   eax, ecx
.error:
    call vga.set_style_error
    call vga.println
    call vga.set_style_normal
    ret

.check:
    call vga.set_style_info
    call vga.print

    mov al, 0x2e                                    ; print "... "
    call vga.print_char
    call vga.print_char
    call vga.print_char
    mov al, 0x20
    call vga.print_char
    ret

.check_ok:
    mov esi, log_string_ok
    call log.success
    ret

.check_error:
    mov esi, log_string_fail
    call log.error
    ret

vga:
; print a string and the crlf sequence
; IN
;   esi: pointer to a null-terminated string
; CLOBBER
;   eax, ecx
.println:
    call vga.print
    call vga.print_crlf
    ret

; move the cursor at the start of the next line
.print_crlf:
    call vga.print_cr
    call vga.print_lf
    ret


; print a string
; IN
;   esi: pointer to a null-terminated string
; CLOBBER
;   eax, ecx
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
;   eax, ecx
.print_char:
    cmp al, 13
    je vga.print_cr

    cmp al, 10
    je vga.print_lf

    mov ah, [vga_cursor.style]

    xor ecx, ecx                                    ; cx = y * WIDTH + x
    imul cx, [vga_cursor.y], VGA_BUFFER_WIDTH
    add cx, [vga_cursor.x]
    shl cx, 1                                       ; cx *= 2 to take style bytes into account

    mov word [ecx + VGA_BUFFER_ADDRESS], ax         ; actual write to the VGA text buffer

    inc word [vga_cursor.x]
    cmp word [vga_cursor.x], VGA_BUFFER_WIDTH       ; if the cursor is going offscreen to the right,
    jae vga.wrap_line                               ; wrap to the next line
    ret
; set the cusror x position to 0
.print_cr:
    mov word [vga_cursor.x], 0
    ret
; lower the cursor y position by 1, scroll up if it reached the bottom of the buffer
.print_lf:
    cmp word [vga_cursor.y], VGA_BUFFER_HEIGHT - 1  ; if the cursor is going offscreen to the bottom,
    jae .scroll_up                                  ; scroll the entire buffer up by one row

    inc word [vga_cursor.y]
    ret
; wrap cursor to the start of the next line when the text overflows to the right
.wrap_line:
    call vga.print_cr
    call vga.print_lf
    ret
; scroll the entire buffer one row up and clear the last line
; CLOBBER
;   eax, ecx
.scroll_up:
    mov ecx, VGA_BUFFER_ADDRESS
.scroll_up_loop:
    mov eax, dword [ecx + VGA_BUFFER_ROW_SIZE]
    mov dword [ecx], eax                            ; shift data one row down, 4 bytes at a time
    add ecx, 4

    cmp ecx, VGA_BUFFER_ADDRESS + (VGA_BUFFER_ROW_SIZE * (VGA_BUFFER_HEIGHT - 1))
    jb vga.scroll_up_loop
; clear the last line with spaces
.scroll_up_clear_loop:
    mov dword [ecx], 0x0f200f20
    add ecx, 4
    cmp ecx, VGA_BUFFER_ADDRESS + (VGA_BUFFER_ROW_SIZE * VGA_BUFFER_HEIGHT)
    jb vga.scroll_up_clear_loop
    ret

; clear the entire buffer
; CLOBBER
;   ecx
.clear:
    mov ecx, VGA_BUFFER_ADDRESS
.clear_loop:
    mov dword [ecx], 0x0f200f20
    add ecx, 4
    cmp ecx, VGA_BUFFER_ADDRESS + (VGA_BUFFER_ROW_SIZE * VGA_BUFFER_HEIGHT)
    jb vga.clear_loop
    ret

; set the style for the vga text mode
; IN
;   al: style byte to set
.set_style:
    mov byte [vga_cursor.style], al
    ret

; set the style to normal
.set_style_normal:
    mov byte [vga_cursor.style], VGA_STYLE_WHITE
    ret

; set the style to info
.set_style_info:
    mov byte [vga_cursor.style], VGA_STYLE_GRAY
    ret

; set the style to ok
.set_style_ok:
    mov byte [vga_cursor.style], VGA_STYLE_BLUE
    ret

; set the style to success
.set_style_success:
    mov byte [vga_cursor.style], VGA_STYLE_GREEN
    ret

; set the style to warning
.set_style_warning:
    mov byte [vga_cursor.style], VGA_STYLE_YELLOW
    ret

; set the style to error
.set_style_error:
    mov byte [vga_cursor.style], VGA_STYLE_RED
    ret

section .data
vga_cursor:
.x:
    dw 0
.y:
    dw 0
.style:
    db VGA_STYLE_WHITE

section .rodata
log_string_ok:
    db "ok",0
log_string_fail:
    db "fail",0

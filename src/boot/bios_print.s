BITS 16

; print a string and a crlf
println:
  call print
  call print_crlf
  ret

  ; does a carriage return and a line feed
print_crlf:
  mov al, 0xd
  call print_char
  mov al, 0xa
  call print_char
  ret

  ; print a string to the teletype output
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
print_char:
  pusha               ;   push all general purpose registers
  mov ah, 0xe         ;   video - teletype output function
  mov bx, 0xf         ;   page 0, white text
  int 0x10            ;   video bios call
  popa                ;   pop all general purpose registers
  ret


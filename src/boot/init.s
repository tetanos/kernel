BITS 16

init:
  mov si, str_hello
  call println

halt:
  cli
  hlt
  jmp halt

; enable the A20 line with the fast A20 method
a20:
  in al, 0x92
  or al, 2
  out 0x92, al


%include "bios_print.s"

str_hello: db "TetanOS Bootloader - Stage Two",0

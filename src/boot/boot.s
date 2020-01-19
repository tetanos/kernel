BITS 16

BOOTSEG equ 0x07c0
INITSEG equ 0x9000
KERNELSEG equ 0x1000
STACKADDR equ 0x400

_start:
  mov [drive_id], dl             ; save the drive id

  cli                           ; no interrupts
  cld                           ; direction = 0

  xor ax, ax
  mov es, ax
  mov ds, ax
  mov ss, ax

  mov ax, BOOTSEG               ; yeet itself to INITSEG << 4
  mov ds, ax                    ; source
  mov ax, INITSEG
  mov es, ax                    ; destination
  xor si, si
  xor di, di
  mov cx, 256                   ; bootloader is 256 word long
  rep movsw

  mov sp, STACKADDR             ; set the stack to some free real estate

  call print_crlf
  mov si, MSG_TITLE
  call println

  call load_init

  mov ax, 0x1000
  mov es, ax
  mov ax, 0x2000
  mov ds, ax
  xor si, si
  xor di, di
  mov cx, 256
  rep movsw

  ; when the ds register is set it can replace the org operation
  xor ax, ax
  mov ds, ax

branch_success:
  call 0x2000

branch_error:
  mov si, MSG_ERROR
  call print
.halt:                  ; put the system to sleep forever
  cli
  hlt
  jmp .halt

load_init:
  mov ah, 2                     ; read sectors from drive
  mov al, 1                     ; read 1 sectors
  mov dh, 0                     ; head 0
  mov dl, [drive_id]
  mov cx, 2                     ; cylinder 0 sector 2
  mov bx, KERNELSEG
  int 0x13
  ret

%include "bios_print.s"

MSG_TITLE: db "Tetanos Bootloader",0
MSG_ERROR: db "Could not load init",13,10,"Halting...",0

drive_id: db 0

times 510-($-$$) db 0   ; padding to offset 510
dw 0xaa55               ; magic bootsector number


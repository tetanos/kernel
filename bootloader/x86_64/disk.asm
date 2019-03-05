%define BLOCK_SIZE 512

sectalign off

%include "bootsector.asm"

stage2:
    %ifdef ARCH_x86_64
        %include "stage2.asm"
    %endif

    align BLOCK_SIZE, db 0
stage2_end:

%ifdef KERNEL
    kernel:
        %defstr KERNEL_BIN %[KERNEL]
        incbin KERNEL_BIN
    .end:

    align BLOCK_SIZE, db 0
%endif


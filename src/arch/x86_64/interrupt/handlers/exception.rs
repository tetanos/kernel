use super::interrupt;
use crate::arch::x86_64::hardware::cpu;
use crate::interrupt_handler;

/// Memory representation of the context during an exception interrupt.
#[derive(Debug, Copy, Clone)]
#[repr(packed)]
struct ExceptionContext {
    registers: cpu::Registers,
    code: usize,
    interrupt_registers: interrupt::InterruptRegisters,
}

impl ExceptionContext {
    fn dump(&self) {
        //        println!("{:#x?}", &self);
    }
}

macro_rules! exception_handler {
    ($name: ident, $context: ident, $callback: block) => {
        #[naked]
        pub unsafe extern fn $name() {
            #[inline(never)]
            unsafe fn handler($context: &ExceptionContext) {
                $callback
            }

            cpu:: Registers::push();

            let rsp: usize;
            asm!("" : "={rsp}"(rsp) : : : "intel", "volatile");

            handler(&*(rsp as *const ExceptionContext));

            cpu::Registers::pop();
            asm!("add rsp, 8" : : : : "intel", "volatile");
            interrupt::ireturn();
        }
    }
}

/// Division by Zero Exception handler
interrupt_handler!(divide_by_zero, context, {
    println!("Divide by zero fault");
    context.dump();
});

/// Debug Exception handler
interrupt_handler!(debug, context, {
    println!("Debug trap");
    context.dump();
});

/// Non Maskable Interrupt Exception handler (NMI)
interrupt_handler!(non_maskable, context, {
    println!("Non-maskable interrupt");
    context.dump();
});

/// Breakpoint Exception handler
interrupt_handler!(breakpoint, context, {
    println!("Breakpoint trap");
    context.dump();
});

/// Overflow Exception handler
interrupt_handler!(overflow, context, {
    println!("Overflow trap");
    context.dump();
});

/// Bound Check Exception handler
interrupt_handler!(bound_check, context, {
    println!("Bound check fault");
    context.dump();
});

/// Invalid Opcode Exception handler
interrupt_handler!(invalid_opcode, context, {
    println!("Invalid Opcode fault");
    context.dump();
});

/// Device Not Available Exception handler
interrupt_handler!(device_not_available, context, {
    println!("Device not available fault");
    context.dump();
});

/// Double Fault Exception handler
exception_handler!(double_fault, context, {
    //println!("Double fault");
    loop {}
    context.dump();
});

/// Invalid TSS Exception handler
exception_handler!(invalid_tss, context, {
    println!("Invalid TSS fault");
    context.dump();
});

/// Segment Not Present Exception handler
exception_handler!(segment_not_present, context, {
    println!("Segment not present fault");
    context.dump();
});

/// Stack Segment Exception handler
exception_handler!(stack_segment, context, {
    println!("Stack segment fault");
    context.dump();
});

/// Protection Exception handler
exception_handler!(protection, context, {
    println!("General protection fault");
    context.dump();
    loop {}
});

/// Page Fault Exception handler
exception_handler!(page, context, {
    let cr2: usize;
    asm!("mov rax, cr2" : "={rax}"(cr2) : : : "intel", "volatile");
    println!("Page fault at {:x}", cr2);
    context.dump();
    loop {}
});

/// Floating Point Exception handler
interrupt_handler!(floating_point, context, {
    println!("Floating point exception");
    context.dump();
});

/// Alignment Check Exception handler
exception_handler!(alignment_check, context, {
    println!("Alignment check fault");
    context.dump();
});

/// Machine Check Exception handler
interrupt_handler!(machine_check, context, {
    println!("Machine check fault");
    context.dump();
});

/// SIMD Exception handler
interrupt_handler!(simd, context, {
    println!("SIMD floating point exception");
    context.dump();
});

/// Virtualization Exception handler
interrupt_handler!(virtualization, context, {
    println!("Virtualization exception");
    context.dump();
});

/// Security Exception handler
exception_handler!(security, context, {
    println!("Security exception");
    context.dump();
});

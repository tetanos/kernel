use super::InterruptRegisters;
use super::Registers;

/// Memory representation of the context during an exception interrupt.
#[derive(Debug, Copy, Clone)]
#[repr(packed)]
struct ExceptionContext {
    registers: Registers,
    code: usize,
    interrupt_registers: InterruptRegisters,
}

impl ExceptionContext {
    fn dump(&self) {
        println!("{:#x?}", &self);
    }
}

macro_rules! exception_handler {
    ($name: ident, $context: ident, $callback: block) => {
        pub unsafe fn $name() {
            unsafe fn handler($context: &ExceptionContext) {
                $callback
            }

            Registers::push();

            let rsp: usize;
            asm!("" : "={rsp}"(rsp) : : : "intel", "volatile");

            handler(&*(rsp as *const ExceptionContext));

            Registers::pop();
            super::ireturn();
        }
    }
}

/// Division by Zero Exception handler
exception_handler!(divide_by_zero, context, {
    context.dump();
});

/// Debug Exception handler
exception_handler!(debug, context, {
    println!("Debug trap");
    context.dump();
});

/// Non Maskable Interrupt Exception handler (NMI)
exception_handler!(non_maskable, context, {
    println!("Non-maskable interrupt");
    context.dump();
});

/// Breakpoint Exception handler
exception_handler!(breakpoint, context, {
    println!("Breakpoint trap");
    context.dump();
});

/// Overflow Exception handler
exception_handler!(overflow, context, {
    println!("Overflow trap");
    context.dump();
});

/// Bound Check Exception handler
exception_handler!(bound_check, context, {
    println!("Bound check fault");
    context.dump();
});

/// Invalid Opcode Exception handler
exception_handler!(invalid_opcode, context, {
    println!("Invalid Opcode fault");
    context.dump();
});

/// Device Not Available Exception handler
exception_handler!(device_not_available, context, {
    println!("Device not available fault");
    context.dump();
});

/// Double Fault Exception handler
exception_handler!(double_fault, context, {
    println!("Double fault");
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
});

/// Page Fault Exception handler
exception_handler!(page, context, {
    println!("Page fault");
    context.dump();
});

/// Floating Point Exception handler
exception_handler!(floating_point, context, {
    println!("Floating point exception");
    context.dump();
});

/// Alignment Check Exception handler
exception_handler!(alignment_check, context, {
    println!("Alignment check fault");
    context.dump();
});

/// Machine Check Exception handler
exception_handler!(machine_check, context, {
    println!("Machine check fault");
    context.dump();
});

/// SIMD Exception handler
exception_handler!(simd, context, {
    println!("SIMD floating point exception");
    context.dump();
});

/// Virtualization Exception handler
exception_handler!(virtualization, context, {
    println!("Virtualization exception");
    context.dump();
});

/// Security Exception handler
exception_handler!(security, context, {
    println!("Security exception");
    context.dump();
});

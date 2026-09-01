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
        println!("{:#x?}", &self);
    }
}

macro_rules! exception_handler {
    ($name: ident, $context: ident, $callback: block) => {
        #[unsafe(naked)]
        pub unsafe extern "C" fn $name() {
            unsafe extern "C" fn handler($context: &ExceptionContext) {
                $callback
            }

            core::arch::naked_asm!(
                "push r15",
                "push r14",
                "push r13",
                "push r12",
                "push rbp",
                "push rbx",
                "push r11",
                "push r10",
                "push r9",
                "push r8",
                "push rsi",
                "push rdi",
                "push rdx",
                "push rcx",
                "push rax",
                "mov rdi, rsp",
                "call {handler}",
                "pop rax",
                "pop rcx",
                "pop rdx",
                "pop rdi",
                "pop rsi",
                "pop r8",
                "pop r9",
                "pop r10",
                "pop r11",
                "pop rbx",
                "pop rbp",
                "pop r12",
                "pop r13",
                "pop r14",
                "pop r15",
                "add rsp, 8",
                "iretq",
                handler = sym handler,
            );
        }
    }
}

// Division by Zero Exception handler
interrupt_handler!(divide_by_zero, context, {
    println!("Divide by zero fault");
    context.dump();
});

// Debug Exception handler
interrupt_handler!(debug, context, {
    println!("Debug trap");
    context.dump();
});

// Non Maskable Interrupt Exception handler (NMI)
interrupt_handler!(non_maskable, context, {
    println!("Non-maskable interrupt");
    context.dump();
});

// Breakpoint Exception handler
interrupt_handler!(breakpoint, context, {
    println!("Breakpoint trap");
    context.dump();
});

// Overflow Exception handler
interrupt_handler!(overflow, context, {
    println!("Overflow trap");
    context.dump();
});

// Bound Check Exception handler
interrupt_handler!(bound_check, context, {
    println!("Bound check fault");
    context.dump();
});

// Invalid Opcode Exception handler
interrupt_handler!(invalid_opcode, context, {
    println!("Invalid Opcode fault");
    context.dump();
});

// Device Not Available Exception handler
interrupt_handler!(device_not_available, context, {
    println!("Device not available fault");
    context.dump();
});

// Double Fault Exception handler
exception_handler!(double_fault, context, {
    println!("Double fault");
    context.dump();
});

// Invalid TSS Exception handler
exception_handler!(invalid_tss, context, {
    println!("Invalid TSS fault");
    context.dump();
});

// Segment Not Present Exception handler
exception_handler!(segment_not_present, context, {
    println!("Segment not present fault");
    context.dump();
});

// Stack Segment Exception handler
exception_handler!(stack_segment, context, {
    println!("Stack segment fault");
    context.dump();
});

// Protection Exception handler
exception_handler!(protection, context, {
    println!("General protection fault");
    context.dump();
});

// Page Fault Exception handler
exception_handler!(page, context, {
    let cr2: usize;
    core::arch::asm!("mov {}, cr2", out(reg) cr2);
    println!("Page fault at {:x}", cr2);
    context.dump();
});

// Floating Point Exception handler
interrupt_handler!(floating_point, context, {
    println!("Floating point exception");
    context.dump();
});

// Alignment Check Exception handler
exception_handler!(alignment_check, context, {
    println!("Alignment check fault");
    context.dump();
});

// Machine Check Exception handler
interrupt_handler!(machine_check, context, {
    println!("Machine check fault");
    context.dump();
});

// SIMD Exception handler
interrupt_handler!(simd, context, {
    println!("SIMD floating point exception");
    context.dump();
});

// Virtualization Exception handler
interrupt_handler!(virtualization, context, {
    println!("Virtualization exception");
    context.dump();
});

// Security Exception handler
exception_handler!(security, context, {
    println!("Security exception");
    context.dump();
});

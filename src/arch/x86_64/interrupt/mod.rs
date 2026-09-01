pub use super::hardware::cpu;

use core::arch::asm;

/// Interrupt Handlers
pub mod handlers;

/// Interrupt descriptor table
pub mod descriptor_table;

/// Registers pushed on the stack during an interrupt.
///
/// When calling `int n`, `int3`, `into` or `int1` instruction, the rflags register is pushed on
/// the stack followed by the code segment and the instruction pointer to allow the `iret`
/// instruction to return to the correct address.
///
/// After an interrupt, the stack should look like this.
///
/// ```
/// +-----------+
/// |  rflags   |
/// +-----------+
/// |    cs     |
/// +-----------+
/// |  old rip  |
/// +-----------+ <- rsp
/// ```
#[derive(Debug, Copy, Clone)]
#[repr(packed)]
pub struct InterruptRegisters {
    rip: usize,
    cs: usize,
    rflags: usize,
}

/// Memory representation of the context during an interrupt request or a system call.
#[derive(Debug, Copy, Clone)]
#[repr(packed)]
pub struct InterruptContext {
    regsisters: cpu::Registers,
    interrupt_registers: InterruptRegisters,
}

impl InterruptContext {
    pub fn dump(&self) {
        //println!("{:#x?}", &self);
    }
}

#[macro_export]
macro_rules! interrupt_handler {
    ($name: ident, $context: ident, $callback: block) => {
        #[unsafe(naked)]
        pub unsafe extern "C" fn $name() {
            unsafe extern "C" fn handler($context: &interrupt::InterruptContext) {
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
                "iretq",
                handler = sym handler,
            );
        }
    };
}

/// Set the interrupt flag.
pub fn enable() {
    unsafe {
        asm!("sti");
    }
}

/// Clear the interrupt flag.
pub fn disable() {
    unsafe {
        asm!("cli");
    }
}

/// Call the system call interrupt.
pub fn syscall() {
    unsafe {
        asm!("int 0x80");
    }
}

/// Trigger the breakpoint trap.
pub fn breakpoint() {
    unsafe {
        asm!("int3");
    }
}

/// Halt the system.
///
/// Using this function will stop the cpu until the next interrupt. It reduce energy consumption.
pub fn halt() {
    unsafe {
        asm!("hlt");
    }
}

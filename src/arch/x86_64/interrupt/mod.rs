pub use super::hardware::cpu::Registers;

/// Interrupt exceptions
pub mod exception;

/// Interrupt requests (IRQ)
pub mod request;

/// Kernel syscalls (int 0x80)
pub mod syscall;

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
    regsisters: Registers,
    interrupt_registers: InterruptRegisters,
}

impl InterruptContext {
    /// Takes a snapshot of the current interruption context and pass it down to the callback
    /// argument.
    pub unsafe fn snapshot(callback: fn(&Self)) {
        Registers::push();

        let rsp: usize;
        asm!("" : "={rsp}"(rsp) : : : "intel", "volatile");

        callback(&*(rsp as *const Self));

        Registers::pop();
    }
}

/// Set the interrupt flag.
pub fn enable() {
    unsafe {
        asm!("sti" : : : : "intel", "volatile");
    }
}

/// Clear the interrupt flag.
pub fn disable() {
    unsafe {
        asm!("cli" : : : : "intel", "volatile");
    }
}

/// Call the system call interrupt.
pub fn syscall() {
    unsafe { asm!("int 0x80" : : : : "intel", "volatile") }
}

/// Trigger the breakpoint trap.
pub fn breakpoint() {
    unsafe { asm!("int3" : : : : "intel", "volatile") }
}

/// Return from the current interrupt.
pub fn ireturn() {
    unsafe { asm!("iretq" : : : : "intel", "volatile") }
}

/// Halt the system.
///
/// Using this function will stop the cpu until the next interrupt. It reduce energy consumption.
pub fn halt() {
    unsafe {
        asm!("hlt" : : : : "intel", "volatile");
    }
}

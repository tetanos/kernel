/// Interrupt exceptions
pub mod exception;

/// Interrupt requests (IRQ)
pub mod request;

/// Kernel syscalls (int 0x80)
pub mod syscall;

/// # Enable interrupts
///
/// set the interrupt flag by executing the `sti` asm instruction.
pub fn enable() {
    unsafe {
        asm!("sti" : : : : "intel" : "volatile");
    }
}

/// # Disable interrupts
///
/// clear the interrupt flag by executing the `cli` asm instruction.
pub fn disable() {
    unsafe {
        asm!("cli" : : : : "intel" : "volatile");
    }
}

/// # Syscall
///
/// Call the system call interrupt.
pub fn syscall() {
    unsafe { asm!("int 0x80" : : : : "intel" : "volatile") }
}

/// # Breakpoint
///
/// Trigger the breakpoint trap.
pub fn breakpoint() {
    unsafe { asm!("int 3" : : : : "intel" : "volatile") }
}

/// # Interrupt Return
///
/// Return from the current interrupt using the `iretq` asm instruction.
pub fn ireturn() {
    unsafe { asm!("iretq" : : : : "intel" : "volatile") }
}

/// # Halt the system
///
/// Using this function will reduce energy consumption. Execute the `hlt` asm instruction.
pub fn halt() {
    unsafe {
        asm!("hlt" : : : : "intel" : "volatile");
    }
}

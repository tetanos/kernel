use super::interrupt;
use crate::arch::x86_64::hardware::cpu;
use crate::interrupt_handler;

/// System Call Interrupt Handler
interrupt_handler!(interrupt, context, {
    println!("System call interrupt");
    context.dump();
});

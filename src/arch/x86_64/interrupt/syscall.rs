use super::InterruptContext;
use super::Registers;
use crate::interrupt_handler;

/// System Call Interrupt Handler
interrupt_handler!(interrupt, context, {
    println!("System call interrupt");
    context.dump();
});

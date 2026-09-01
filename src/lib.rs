//! # The TetanOS Kernel
//!
//! An attempt at making a operating system.
//! Make sure you are vaccinated before using.

#![allow(unused_attributes)]
#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

#[macro_use]
mod vga;

/// Architecture dependent modules
pub mod arch;
pub use arch::*;

/// Architecture independent modules
pub mod common;

/// Panic Handlers
pub mod panic;

/// Entry point, called from arch start module
pub fn kernel_main() -> ! {
    vga::ferris_say("This is TetanOS");
    println!("Be careful, it's kinda rusty in here.");

    loop {
        interrupt::halt();
    }
}

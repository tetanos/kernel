//! # The TetanOS Kernel
//!
//! An attempt at making a operating system.
//! Make sure you are vaccinated before using.

#![allow(unused_attributes)]
#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
#![cfg_attr(test, allow(unused_imports))]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;

mod gdt;
mod interrupts;
mod serial;
mod vga;

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hi!");
    vga::ferris_say("This is TetanOS");
    println!("Be careful, it's kinda rusty in here.");
    init();
}

pub fn init() -> ! {
    gdt::init();
    interrupts::init();

    use interrupts::PICS;
    unsafe { PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();
    vga::term::TERM.lock().init();

    hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    hlt_loop()
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

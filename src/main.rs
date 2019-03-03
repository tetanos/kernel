#![no_std]
#![no_main]

mod vga;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hi!");
    println!("This is TetanOS.");
    print!("Be careful, it's kinda rusty in here.");

    loop {}
}

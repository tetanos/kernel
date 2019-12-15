use core::panic::PanicInfo;

use super::interrupt;

#[cfg(not(test))]
#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn rust_eh_personality() {}

/// # Panic Handler
///
/// Print a panic info object and halt, something went terribly wrong at this point.
#[cfg(not(test))]
#[panic_handler]
#[no_mangle]
pub extern "C" fn rust_begin_unwind(info: &PanicInfo) -> ! {
    println!("KERNEL PANIC: {}", info);

    loop {
        interrupt::halt();
    }
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn _Unwind_Resume() -> ! {
    loop {
        interrupt::halt();
    }
}

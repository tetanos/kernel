use core::panic::PanicInfo;

use super::interrupt;

/// # Panic Handler
///
/// Print a panic info object and halt, something went terribly wrong at this point.
#[cfg(not(test))]
#[panic_handler]
pub fn rust_begin_unwind(info: &PanicInfo) -> ! {
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

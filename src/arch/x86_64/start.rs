use super::descriptor_table;
use crate::kernel_main;

/// Entry point of the kernel for the x86_64 architecture.
#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    descriptor_table::interrupt::init();
    //    descriptor_table::global::init();

    println!("test");
    super::interrupt::breakpoint();
    println!("t123est");

    super::interrupt::syscall();
    kernel_main()
}

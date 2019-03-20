use super::hardware::io::programmable_interrupt_controller as pic;
use super::interrupt;
use super::segmentation;
use crate::kernel_main;

/// Entry point of the kernel for the x86_64 architecture.
#[no_mangle]
pub unsafe extern "C" fn kernel_start() -> ! {
    segmentation::global_descriptor_table::init();
    interrupt::descriptor_table::init();

    pic::init();
    interrupt::enable();

    kernel_main();
}

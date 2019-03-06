use super::descriptor_table;
use crate::kernel_main;

#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    descriptor_table::global::init();
    descriptor_table::interrupt::init();

    kernel_main()
}

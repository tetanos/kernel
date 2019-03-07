use super::descriptor_table;
use crate::kernel_main;

use super::hardware::cpu::*;

#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    //    descriptor_table::global::init();
    descriptor_table::interrupt::init();

    println!(
        "{:?}",
        Regsiters {
            rax: 123,
            rbx: 123,
            rcx: 123,
            rdx: 123,
            rsi: 123,
            rdi: 123,
            rbp: 123,
            rsp: 123,
            r8: 123,
            r9: 123,
            r10: 123,
            r11: 123,
            r12: 123,
            r13: 123,
            r14: 123,
            r15: 123,
        }
    );
    super::interrupt::syscall();
    kernel_main()
}

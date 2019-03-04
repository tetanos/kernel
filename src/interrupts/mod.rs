use lazy_static::lazy_static;
use x86_64::structures::idt::{ExceptionStackFrame, InterruptDescriptorTable};

use crate::gdt;
use crate::println;
use crate::vga;
use crate::vga::buffer::Color;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();

        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe {
            idt.double_fault
                .set_handler_fn(double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }

        idt
    };
}

pub fn init() {
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: &mut ExceptionStackFrame) {
    vga::set_foreground_color(Color::Red);
    println!("Breakpoint!\n{:#?}", stack_frame);
    vga::set_foreground_color(Color::White);
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: &mut ExceptionStackFrame,
    _error_code: u64,
) {
    vga::set_foreground_color(Color::Red);
    println!("Double Fault!\n{:#?}", stack_frame);
    vga::set_foreground_color(Color::White);
    loop {}
}

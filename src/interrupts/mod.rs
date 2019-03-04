use lazy_static::lazy_static;
use x86_64::structures::idt::{ExceptionStackFrame, InterruptDescriptorTable};

use crate::println;
use crate::vga;
use crate::vga::buffer::Color;

use pic8259_simple::ChainedPics;
use spin;

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static PICS: spin::Mutex<ChainedPics> =
    spin::Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt
    };
}

pub fn init() {
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: &mut ExceptionStackFrame) {
    vga::set_foreground_color(Color::Red);
    println!("{:#?}", stack_frame);
    vga::set_foreground_color(Color::White);
}

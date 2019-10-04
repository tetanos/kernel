use super::interrupt;
use crate::arch::x86_64::hardware::cpu;
use crate::arch::x86_64::hardware::io::programmable_interrupt_controller as pic;
use crate::arch::x86_64::hardware::io::ProgrammedIO;
use crate::common::IO;
use crate::interrupt_handler;

macro_rules! irq_handler {
    ($name: ident, $callback: block) => {
        #[naked]
        pub unsafe extern "C" fn $name() {
            #[inline(never)]
            unsafe fn handler() {
                $callback
            }

            handler();

            interrupt::ireturn();
        }
    };
}

pub unsafe fn acknowledge(irq: u8) {
    if irq < 16 {
        pic::MASTER.acknowledge();
        if irq >= 8 {
            pic::SLAVE.acknowledge();
        }
    }
}

// Programmable Interrupt Timer
irq_handler!(programmable_interrupt_timer, {
    //println!("pit");
    pic::MASTER.acknowledge();
});

// Keyboard Interrupt Request
irq_handler!(keyboard, {
    use lazy_static::lazy_static;
    use pc_keyboard::{layouts, DecodedKey, Keyboard, ScancodeSet1};
    use spin::Mutex;

    lazy_static! {
        static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> =
            Mutex::new(Keyboard::new(layouts::Us104Key, ScancodeSet1));
    }

    let mut keyboard = KEYBOARD.lock();
    let io: ProgrammedIO<u8> = ProgrammedIO::new(0x60);

    let scancode: u8 = io.read();
    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            match key {
                DecodedKey::Unicode(character) => {
                    print!("{}", character);
                }
                DecodedKey::RawKey(key) => print!("{:?}", key),
            }
        }
    }

    acknowledge(1);
});

// Cascade
interrupt_handler!(cascade, context, {
    context.dump();
});

// Com2
interrupt_handler!(com2, context, {
    context.dump();
});

// Com1
interrupt_handler!(com1, context, {
    context.dump();
});

// Lpt2
interrupt_handler!(lpt2, context, {
    context.dump();
});

// Floppy Interrupt Request
interrupt_handler!(floppy, context, {
    context.dump();
});

// Lpt1
interrupt_handler!(lpt1, context, {
    context.dump();
});

// Real time cmos clock Interrupt Request
interrupt_handler!(cmos, context, {
    context.dump();
});

// Pci1
interrupt_handler!(pci1, context, {
    context.dump();
});

// Pci2
interrupt_handler!(pci2, context, {
    context.dump();
});

// Pci3
interrupt_handler!(pci3, context, {
    context.dump();
});

// PS2 Mouse Interrupt Request
interrupt_handler!(ps2_mouse, context, {
    context.dump();
});

// FPU
interrupt_handler!(fpu, context, {
    context.dump();
});

// Primary ATA Disk Interrupt Request
interrupt_handler!(ata1, context, {
    context.dump();
});

// Secondary ATA Disk Interrupt Request
interrupt_handler!(ata2, context, {
    context.dump();
});

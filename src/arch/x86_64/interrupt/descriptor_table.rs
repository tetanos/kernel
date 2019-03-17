use core::mem::size_of;

use super::super::segmentation;
use super::handlers::*;

/// A reference to the idt object in memory.
static mut IDT_REF: segmentation::DescriptorTablePointer<Entry> =
    segmentation::DescriptorTablePointer {
        limit: 0,
        address: 0 as *const Entry,
    };

/// table containing pointers to handler function of exceptions, interrupt and syscalls
static mut IDT: [Entry; 256] = [Entry::new(); 256];

pub unsafe fn init() {
    IDT_REF.limit = (IDT.len() * size_of::<Entry>() - 1) as u16;
    IDT_REF.address = IDT.as_ptr() as *const Entry;

    // Exceptions interrupt
    IDT[0].set_handler(exception::divide_by_zero);
    IDT[1].set_handler(exception::debug);
    IDT[2].set_handler(exception::non_maskable);
    IDT[3].set_handler(exception::breakpoint);
    IDT[4].set_handler(exception::overflow);
    IDT[5].set_handler(exception::bound_check);
    IDT[6].set_handler(exception::invalid_opcode);
    IDT[7].set_handler(exception::device_not_available);
    IDT[8].set_handler(exception::double_fault);
    // 9 removed: IDT[9].set_handler(exception::coprocessor_segment_overrun);
    IDT[10].set_handler(exception::invalid_tss);
    IDT[11].set_handler(exception::segment_not_present);
    IDT[12].set_handler(exception::stack_segment);
    IDT[13].set_handler(exception::protection);
    IDT[14].set_handler(exception::page);
    // 15 reserved
    IDT[16].set_handler(exception::floating_point);
    IDT[17].set_handler(exception::alignment_check);
    IDT[18].set_handler(exception::machine_check);
    IDT[19].set_handler(exception::simd);
    IDT[20].set_handler(exception::virtualization);
    // 21 -> 29 reserved
    IDT[30].set_handler(exception::security);
    // 31 reserved

    // Interrupt request
    IDT[32].set_handler(request::programmable_interrupt_timer);
    IDT[33].set_handler(request::keyboard);
    IDT[34].set_handler(request::cascade);
    IDT[35].set_handler(request::com2);
    IDT[36].set_handler(request::com1);
    IDT[37].set_handler(request::lpt2);
    IDT[38].set_handler(request::floppy);
    IDT[39].set_handler(request::lpt1);
    IDT[40].set_handler(request::cmos);
    IDT[41].set_handler(request::pci1);
    IDT[42].set_handler(request::pci2);
    IDT[43].set_handler(request::pci3);
    IDT[44].set_handler(request::ps2_mouse);
    IDT[45].set_handler(request::fpu);
    IDT[46].set_handler(request::ata1);
    IDT[47].set_handler(request::ata2);

    // System call
    IDT[0x80].set_handler(syscall::interrupt);

    segmentation::lidt(&IDT_REF);
}

/// An entry in the interrupt descriptor table.
#[derive(Copy, Clone, Debug)]
#[repr(packed)]
struct Entry {
    offset_l: u16,
    selector: u16,
    zero1: u8,
    attribute: u8,
    offset_m: u16,
    offset_h: u32,
    zero2: u32,
}

impl Entry {
    const fn new() -> Self {
        Entry {
            offset_l: 0,
            selector: 0,
            zero1: 0,
            attribute: 0,
            offset_m: 0,
            offset_h: 0,
            zero2: 0,
        }
    }

    fn set_offset(&mut self, selector: u16, address: usize) {
        self.selector = selector;
        self.offset_l = address as u16;
        self.offset_m = (address >> 16) as u16;
        self.offset_h = (address >> 32) as u32;
    }

    fn set_handler(&mut self, handler: unsafe extern "C" fn()) {
        self.attribute = Attribute::new(
            true,
            segmentation::RingLevel::Zero,
            false,
            AttributeType::Interrupt,
        )
        .0;

        // this points to the kernel code segment since it's 8 bytes from the start of GDT
        self.set_offset(8, handler as usize);
    }
}

/// Attribute type of the descriptor.
#[allow(dead_code)]
#[repr(u8)]
enum AttributeType {
    /// Interrupt Gate
    Interrupt = 0xe,
    /// Trap Gate
    Trap = 0xf,
}

/// Represent the type attribute bits of the interrupt descriptor.
#[derive(Copy, Clone, Debug)]
struct Attribute(u8);

impl Attribute {
    const fn new(
        present: bool,
        ring: segmentation::RingLevel,
        storage_segment: bool,
        attribute_type: AttributeType,
    ) -> Self {
        Attribute(
            (present as u8) << 7
                | (ring as u8) << 5
                | (storage_segment as u8) << 5
                | attribute_type as u8,
        )
    }
}

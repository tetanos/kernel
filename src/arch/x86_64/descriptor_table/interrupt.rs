use core::mem::size_of;

use super::DescriptorTablePointer;
use super::RingLevel;
use crate::arch::x86_64::interrupt::*;

/// # Interrupt Descriptor Table Reference
///
/// A reference to the idt object in memory.
static mut IDT_REF: DescriptorTablePointer<Descriptor> = DescriptorTablePointer {
    limit: 0,
    address: 0 as *const Descriptor,
};

/// # Interrupt Descriptor Table
///
/// This is a table containing pointers to handler function of exceptions, interrupt, syscalls,
/// etc.
static mut IDT: [Descriptor; 256] = [Descriptor::new(); 256];

pub unsafe fn init() {
    IDT_REF.limit = (IDT.len() * size_of::<DescriptorEntry>() - 1) as u16;
    IDT_REF.address = IDT.as_ptr() as *const DescriptorEntry;

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

    super::lidt(&IDT_REF);
}

/// # Interrupt Descriptor Attribute Type
#[allow(dead_code)]
#[repr(u8)]
enum DescriptorAttributeType {
    /// Interrupt Gate
    Interrupt = 0xe,
    /// Trap Gate
    Trap = 0xf,
}

/// Interrupt Descriptor Attribute
///
/// Represent the type attribute bits of the interrupt descriptor.
#[derive(Copy, Clone, Debug)]
struct DescriptorAttribute(u8);

impl DescriptorAttribute {
    const fn new(
        present: bool,
        ring: RingLevel,
        storage_segment: bool,
        attribute_type: DescriptorAttributeType,
    ) -> Self {
        DescriptorAttribute(
            (present as u8) << 7
                | (ring as u8) << 5
                | (storage_segment as u8) << 5
                | attribute_type as u8,
        )
    }
}

/// Interrupt Descriptor Entry
///
/// An entry in the interrupt descriptor table.
#[derive(Copy, Clone, Debug)]
#[repr(packed)]
struct Descriptor {
    offset_l: u16,
    selector: u16,
    zero1: u8,
    attribute: u8,
    offset_m: u16,
    offset_h: u32,
    zero2: u32,
}

impl Descriptor {
    const fn new() -> Self {
        Descriptor {
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

    fn set_handler(&mut self, handler: unsafe fn()) {
        self.attribute = DescriptorAttribute::new(
            true,
            RingLevel::Zero,
            false,
            DescriptorAttributeType::Interrupt,
        )
        .0;
        self.set_offset(8, handler as usize);
    }
}

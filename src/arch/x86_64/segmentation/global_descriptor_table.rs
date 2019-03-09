use core::mem::size_of;

use super::task_state_segment::{self, TaskStateSegment, TSS};
use super::DescriptorTablePointer;
use super::GlobalDescriptorType as Type;
use super::RingLevel;
use super::SegmentSelector;
use crate::arch::x86_64::hardware::cpu;
use crate::arch::x86_64::segmentation;

/// A reference to the gdt object in memory
static mut GDT_REF: DescriptorTablePointer<Entry> = DescriptorTablePointer {
    limit: 0,
    address: 0 as *const Entry,
};

/// Table containing information about memory segments.
static mut GDT: [Entry; 7] = [
    // Zero
    Entry::new(
        0,
        0,
        Access::new(false, RingLevel::Zero, false, false, false, false, false),
        Flags(cpu::Mode::Real, Granularity::Byte),
    ),
    // Kernel Code
    Entry::new(
        0,
        0,
        Access::new(true, RingLevel::Zero, true, true, false, true, false),
        Flags(cpu::Mode::Long, Granularity::Byte),
    ),
    // Kernel Data
    Entry::new(
        0,
        0,
        Access::new(true, RingLevel::Zero, true, false, false, true, false),
        Flags(cpu::Mode::Long, Granularity::Byte),
    ),
    // User Code
    Entry::new(
        0,
        0,
        Access::new(true, RingLevel::Three, true, true, false, true, false),
        Flags(cpu::Mode::Long, Granularity::Byte),
    ),
    // User Data
    Entry::new(
        0,
        0,
        Access::new(true, RingLevel::Three, true, false, false, true, false),
        Flags(cpu::Mode::Long, Granularity::Byte),
    ),
    // Task State Segment
    Entry::new(
        0,
        0,
        Access::new(true, RingLevel::Three, false, false, false, false, false),
        Flags(cpu::Mode::Long, Granularity::Byte),
    ),
    // Task State Segment High
    Entry::new(
        0,
        0,
        Access::new(false, RingLevel::Zero, false, false, false, false, false),
        Flags(cpu::Mode::Real, Granularity::Byte),
    ),
];

/// Loads the gdt into memory
pub unsafe fn init() {
    GDT_REF.limit = (GDT.len() * size_of::<Entry>() - 1) as u16;
    GDT_REF.address = GDT.as_ptr() as *const Entry;

    segmentation::lgdt(&GDT_REF);

    GDT[Type::TaskState as usize].set_offset(&TSS as *const _ as u32);
    GDT[Type::TaskState as usize].set_limit(size_of::<TaskStateSegment>() as u32);

    // idk wtf to put in there
    TSS.rsp[0] = 0xdeadbeef as u64;

    segmentation::load_code_segment(SegmentSelector::new(Type::KernelCode, RingLevel::Zero));
    segmentation::load_data_segment(SegmentSelector::new(Type::KernelData, RingLevel::Zero));
    segmentation::load_extra_segment(SegmentSelector::new(Type::KernelData, RingLevel::Zero));
    segmentation::load_g_segment(SegmentSelector::new(Type::KernelData, RingLevel::Zero));
    segmentation::load_stack_segment(SegmentSelector::new(Type::KernelData, RingLevel::Zero));

    println!("tr");
    //    task_state_segment::load_task_register(SegmentSelector::new(Type::TaskState, RingLevel::Zero));
    println!("tr-done");
}

/// Entry in the GDT.
///
/// For historic reason this structure is using a weird layout.
#[derive(Copy, Clone, Debug)]
#[repr(packed)]
pub struct Entry {
    limit_l: u16,
    offset_l: u16,
    offset_m: u8,
    access: u8,
    flags_limit_h: u8,
    offset_h: u8,
}

impl Entry {
    const fn new(offset: u32, limit: u32, access: Access, flags: Flags) -> Self {
        Entry {
            limit_l: limit as u16,
            offset_l: offset as u16,
            offset_m: (offset >> 16) as u8,
            access: access.0,
            flags_limit_h: flags.value() & 0xf0 | (limit >> 16) as u8 & 0x0f,
            offset_h: (offset >> 24) as u8,
        }
    }

    fn set_offset(&mut self, value: u32) {
        self.offset_l = value as u16;
        self.offset_m = (value >> 16) as u8;
        self.offset_h = (value >> 24) as u8;
    }

    fn set_limit(&mut self, value: u32) {
        self.limit_l = value as u16;
        self.flags_limit_h = self.flags_limit_h & 0xf0 | ((value >> 16) as u8) & 0x0f;
    }
}

/// Represent the access bits of the segment descriptor.
#[derive(Copy, Clone, Debug)]
struct Access(u8);

impl Access {
    const fn new(
        present: bool,
        ring: RingLevel,
        system: bool,
        executable: bool,
        conforming: bool,
        privileged: bool,
        dirty: bool,
    ) -> Self {
        Access(
            (present as u8) << 7
                | (ring as u8 & 3) << 5
                | (system as u8) << 4
                | (executable as u8) << 3
                | (conforming as u8) << 2
                | (privileged as u8) << 1
                | dirty as u8,
        )
    }
}

/// Represent the flag bits of the segment descriptor.
#[derive(Copy, Clone, Debug)]
struct Flags(cpu::Mode, Granularity);

impl Flags {
    const fn value(&self) -> u8 {
        ((self.0 as u8 & 1) << 1 | (self.0 as u8 & 2) >> 1) << 5 | (self.1 as u8) << 7
    }
}

/// Unit for the limit attribute of a segment.
/// * Byte mode uses 1 byte blocks
/// * Page mode uses 4kb blocks
#[allow(dead_code)]
#[derive(Copy, Clone, Debug)]
#[repr(u8)]
enum Granularity {
    Byte = 0,
    Page = 1,
}

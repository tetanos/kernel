use core::mem::size_of;

use super::hardware::cpu;
use super::segmentation::*;
use super::task_state::{self, TaskStateSegment, TSS};
use super::DescriptorTablePointer;
use super::RingLevel;

/// A reference to the gdt object in memory
static mut GDT_REF: DescriptorTablePointer<Descriptor> = DescriptorTablePointer {
    limit: 0,
    address: 0 as *const Descriptor,
};

/// Table containing information about memory segments.
static mut GDT: [Descriptor; 9] = [
    // Zero
    Descriptor::new(
        0,
        0,
        Access::new(false, RingLevel::Zero, false, false, false, false, false),
        Flag(cpu::Mode::Real, Granularity::Byte),
    ),
    // Kernel Code
    Descriptor::new(
        0,
        0,
        Access::new(true, RingLevel::Zero, true, true, false, true, false),
        Flag(cpu::Mode::Long, Granularity::Byte),
    ),
    // Kernel Data
    Descriptor::new(
        0,
        0,
        Access::new(true, RingLevel::Zero, true, false, false, true, false),
        Flag(cpu::Mode::Long, Granularity::Byte),
    ),
    // Kernel Thread Local Storage
    Descriptor::new(
        0,
        0,
        Access::new(true, RingLevel::Zero, true, false, false, true, false),
        Flag(cpu::Mode::Long, Granularity::Byte),
    ),
    // User Code
    Descriptor::new(
        0,
        0,
        Access::new(true, RingLevel::Three, true, true, false, true, false),
        Flag(cpu::Mode::Long, Granularity::Byte),
    ),
    // User Data
    Descriptor::new(
        0,
        0,
        Access::new(true, RingLevel::Three, true, false, false, true, false),
        Flag(cpu::Mode::Long, Granularity::Byte),
    ),
    // User Thread Local Storage
    Descriptor::new(
        0,
        0,
        Access::new(true, RingLevel::Three, true, false, false, true, false),
        Flag(cpu::Mode::Long, Granularity::Byte),
    ),
    // Task State Segment
    Descriptor::new(
        0,
        0,
        Access::new(true, RingLevel::Three, false, false, false, false, false),
        Flag(cpu::Mode::Long, Granularity::Byte),
    ),
    // Task State Segment High
    Descriptor::new(
        0,
        0,
        Access::new(false, RingLevel::Zero, false, false, false, false, false),
        Flag(cpu::Mode::Real, Granularity::Byte),
    ),
];

/// Loads the gdt into memory
pub unsafe fn init() {
    GDT_REF.limit = (GDT.len() * size_of::<Descriptor>() - 1) as u16;
    GDT_REF.address = GDT.as_ptr() as *const Descriptor;

    super::lgdt(&GDT_REF);

    GDT[Type::TaskState as usize].set_offset(&TSS as *const _ as u32);
    GDT[Type::TaskState as usize].set_limit(size_of::<TaskStateSegment>() as u32);

    // idk wtf to put in there
    TSS.rsp[0] = 0x5555555555 as u64;

    load_cs(Selector::new(Type::KernelCode, RingLevel::Zero));
    load_ds(Selector::new(Type::KernelData, RingLevel::Zero));
    load_es(Selector::new(Type::KernelData, RingLevel::Zero));
    load_fs(Selector::new(Type::KernelThreadLocal, RingLevel::Zero));
    load_gs(Selector::new(Type::KernelData, RingLevel::Zero));
    load_ss(Selector::new(Type::KernelData, RingLevel::Zero));

    println!("tr");
    task_state::load_tr(Selector::new(Type::TaskState, RingLevel::Zero));
    println!("tr-done");
}

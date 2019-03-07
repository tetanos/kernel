use core::mem::size_of;

use super::hardware::cpu;
use super::segmentation::*;
use super::DescriptorTablePointer;
use super::RingLevel;

/// # Global Table Descriptor Reference
///
/// A reference to the gdt object in memory
static mut GDT_REF: DescriptorTablePointer<Descriptor> = DescriptorTablePointer {
    limit: 0,
    address: 0 as *const Descriptor,
};

/// # Global Descriptor Table
///
/// This is the table containing information about memory segment.
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

/// Initialization
///
/// Loads the gdt into memory
pub unsafe fn init() {
    GDT_REF.limit = (GDT.len() * size_of::<Descriptor>() - 1) as u16;
    GDT_REF.address = GDT.as_ptr() as *const Descriptor;

    super::lgdt(&GDT_REF);

    // We can now access our TSS, which is a thread local
    // GDT[GDT_TSS].set_offset(&TSS as *const _ as u32);
    // GDT[GDT_TSS].set_limit(mem::size_of::<TaskState>() as u32);

    // set tss stack

    //set_cs(Selector::new(Type::KernelCode, RingLevel::Zero));
    load_ds(Selector::new(Type::KernelData, RingLevel::Zero));
    load_es(Selector::new(Type::KernelData, RingLevel::Zero));
    load_fs(Selector::new(Type::KernelThreadLocal, RingLevel::Zero));
    load_gs(Selector::new(Type::KernelData, RingLevel::Zero));
    load_ss(Selector::new(Type::KernelData, RingLevel::Zero));

    // Load the task register
    //task_state::load_tr(Selector::new(Type::TaskState, RingLevel::Zero));
}

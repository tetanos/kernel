use core::mem::size_of;

use super::hardware::cpu;
use super::DescriptorTablePointer;
use super::RingLevel;

/// # Global Table Descriptor Reference
///
/// A reference to the gdt object in memory
static mut GDT_REF: DescriptorTablePointer<SegmentDescriptor> = DescriptorTablePointer {
    limit: 0,
    address: 0 as *const SegmentDescriptor,
};

/// # Global Descriptor Table
///
/// This is the table containing information about memory segment.
static mut GDT: [SegmentDescriptor; 9] = [
    SegmentDescriptor::new(
        0,
        0,
        SegmentAccess::new(false, RingLevel::Zero, false, false, false, false, false),
        SegmentFlag(cpu::Mode::Real, SegmentGranularity::Byte),
    ),
    SegmentDescriptor::new(
        0,
        0,
        SegmentAccess::new(true, RingLevel::Zero, true, true, false, true, false),
        SegmentFlag(cpu::Mode::Long, SegmentGranularity::Byte),
    ),
    SegmentDescriptor::new(
        0,
        0,
        SegmentAccess::new(true, RingLevel::Zero, true, false, false, true, false),
        SegmentFlag(cpu::Mode::Long, SegmentGranularity::Byte),
    ),
    SegmentDescriptor::new(
        0,
        0,
        SegmentAccess::new(true, RingLevel::Zero, true, false, false, true, false),
        SegmentFlag(cpu::Mode::Long, SegmentGranularity::Byte),
    ),
    SegmentDescriptor::new(
        0,
        0,
        SegmentAccess::new(true, RingLevel::Three, true, true, false, true, false),
        SegmentFlag(cpu::Mode::Long, SegmentGranularity::Byte),
    ),
    SegmentDescriptor::new(
        0,
        0,
        SegmentAccess::new(true, RingLevel::Three, true, false, false, true, false),
        SegmentFlag(cpu::Mode::Long, SegmentGranularity::Byte),
    ),
    SegmentDescriptor::new(
        0,
        0,
        SegmentAccess::new(true, RingLevel::Three, true, false, false, true, false),
        SegmentFlag(cpu::Mode::Long, SegmentGranularity::Byte),
    ),
    SegmentDescriptor::new(
        0,
        0,
        SegmentAccess::new(true, RingLevel::Three, false, false, false, false, false),
        SegmentFlag(cpu::Mode::Long, SegmentGranularity::Byte),
    ),
    SegmentDescriptor::new(
        0,
        0,
        SegmentAccess::new(false, RingLevel::Zero, false, false, false, false, false),
        SegmentFlag(cpu::Mode::Real, SegmentGranularity::Byte),
    ),
];

/// Initialization
///
/// Loads the gdt into memory
pub unsafe fn init() {
    GDT_REF.limit = (GDT.len() * size_of::<SegmentDescriptor>() - 1) as u16;
    GDT_REF.address = GDT.as_ptr() as *const SegmentDescriptor;

    super::lgdt(&GDT_REF);
}

/// # Segment Type
///
/// Type associated with a memory segment entry in the global descriptor table
#[allow(dead_code)]
#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum SegmentType {
    Null = 0,
    KernelCode = 1,
    KernelData = 2,
    KernelThreadLocalStorage = 3,
    UserCode = 4,
    UserData = 5,
    UserThreadLocalStorage = 6,
    TaskState = 7,
    /// The task state segment must be 16 bytes long
    TaskStateHigh = 8,
}

/// # Memory Segment Entry
///
/// Entry in the global , for historic reason this structure is using a weird layout.
///
#[derive(Copy, Clone, Debug)]
#[repr(packed)]
struct SegmentDescriptor {
    limit_l: u16,
    address_l: u16,
    address_m: u8,
    access: u8,
    flags_limit_h: u8,
    address_h: u8,
}

impl SegmentDescriptor {
    const fn new(address: u32, limit: u32, access: SegmentAccess, flags: SegmentFlag) -> Self {
        SegmentDescriptor {
            limit_l: limit as u16,
            address_l: address as u16,
            address_m: (address >> 16) as u8,
            access: access.0,
            flags_limit_h: flags.value() & 0xf0 | (limit >> 16) as u8 & 0x0f,
            address_h: (address >> 24) as u8,
        }
    }
}

/// # Segment Access
///
/// Represent the access bits of the segment descriptor.
#[derive(Copy, Clone, Debug)]
struct SegmentAccess(u8);

impl SegmentAccess {
    const fn new(
        present: bool,
        ring: RingLevel,
        system: bool,
        executable: bool,
        conforming: bool,
        privileged: bool,
        dirty: bool,
    ) -> Self {
        SegmentAccess(
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

/// # Segment Flag
///
/// Represent the flag bits of the segment descriptor.
#[derive(Copy, Clone, Debug)]
struct SegmentFlag(cpu::Mode, SegmentGranularity);

impl SegmentFlag {
    const fn value(&self) -> u8 {
        ((self.0 as u8 & 1) << 1 | (self.0 as u8 & 2) >> 1) << 5 | (self.1 as u8) << 7
    }
}

/// Granularity of the limit of a segment descriptor. Byte granularity represent 1 byte blocks and
/// page granularity represents 4kb blocks
#[allow(dead_code)]
#[derive(Copy, Clone, Debug)]
#[repr(u8)]
enum SegmentGranularity {
    Byte = 0,
    Page = 1,
}

use core::arch::asm;
use core::mem::size_of;

pub mod global_descriptor_table;

pub mod task_state_segment;

/// Privilege level required to interact with a descriptor segment.
///
/// Lower is better.
#[allow(dead_code)]
#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum RingLevel {
    Zero = 0,
    One = 1,
    Two = 2,
    Three = 3,
}

/// Type associated with a memory segment entry in the GDT
#[allow(dead_code)]
#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum GlobalDescriptorType {
    Null = 0,
    KernelCode = 1,
    KernelData = 2,
    TaskState = 3,
}
//pub enum GlobalDescriptorType {
//    Null = 0,
//    KernelCode = 1,
//    KernelData = 2,
//    UserCode = 3,
//    UserData = 4,
//    TaskState = 5,
//    /// The task state segment must be 16 bytes long
//    TaskStateHigh = 6,
//}

/// Index of a segment in the GDT.
pub struct SegmentSelector(pub u16);

impl SegmentSelector {
    pub const fn new(index: GlobalDescriptorType, ring: RingLevel) -> Self {
        SegmentSelector(((index as u8) << 3 | ring as u8) as u16)
    }
}

/// Load a segment into the code segment register.
///
/// TODO: cleanup this mess
pub unsafe fn load_code_segment(selector: SegmentSelector) {
    asm!(
        "push {sel}",
        "lea {tmp}, [rip + 2f]",
        "push {tmp}",
        "retfq",
        "2:",
        sel = in(reg) u64::from(selector.0),
        tmp = out(reg) _,
    );
}

/// Load a segment into the stack segment register.
pub unsafe fn load_stack_segment(selector: SegmentSelector) {
    asm!("mov ss, {0:x}", in(reg) selector.0);
}

/// Load a segment into the data segment register.
pub unsafe fn load_data_segment(selector: SegmentSelector) {
    asm!("mov ds, {0:x}", in(reg) selector.0);
}

/// Load a segment into the extra segment register.
pub unsafe fn load_extra_segment(selector: SegmentSelector) {
    asm!("mov es, {0:x}", in(reg) selector.0);
}

/// Load a segment into the F segment register.
pub unsafe fn load_f_segment(selector: SegmentSelector) {
    asm!("mov fs, {0:x}", in(reg) selector.0);
}

/// Load a segment into the G segment register.
pub unsafe fn load_g_segment(selector: SegmentSelector) {
    asm!("mov gs, {0:x}", in(reg) selector.0);
}

/// Represent a descriptor table into memory.
#[repr(packed)]
pub struct DescriptorTablePointer<EntryType> {
    pub limit: u16,
    pub address: *const EntryType,
}

impl<T> DescriptorTablePointer<T> {
    pub fn new(table: &T) -> Self {
        let entry_length = size_of::<T>() - 1;
        assert!(entry_length < 0x10000);
        DescriptorTablePointer {
            limit: entry_length as u16,
            address: table as *const T,
        }
    }
}

/// Load the global offset table into memory.
pub fn lgdt<T>(gdt: &DescriptorTablePointer<T>) {
    unsafe {
        asm!("lgdt [{}]", in(reg) gdt as *const DescriptorTablePointer<T>);
    }
}

/// Load the local descriptor table into memory.
pub fn lldt<T>(ldt: &DescriptorTablePointer<T>) {
    unsafe {
        asm!("lldt [{}]", in(reg) ldt as *const DescriptorTablePointer<T>);
    }
}

/// Load the interrupt descriptor table into memory.
pub fn lidt<T>(idt: &DescriptorTablePointer<T>) {
    unsafe {
        asm!("lidt [{}]", in(reg) idt as *const DescriptorTablePointer<T>);
    }
}

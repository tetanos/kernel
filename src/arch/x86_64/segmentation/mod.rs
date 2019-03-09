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
    UserCode = 3,
    UserData = 4,
    TaskState = 5,
    /// The task state segment must be 16 bytes long
    TaskStateHigh = 6,
}

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
    //    asm!("movw $0, %cs " :: "r"(selector.0) : "memory" : "volatile");
    #[inline(always)]
    unsafe fn far_jump_to(sel: SegmentSelector) {
        asm!("pushq $0; \
              leaq  1f(%rip), %rax; \
              pushq %rax; \
              lretq; \
              1:" :: "ri" (u64::from(sel.0)) : "rax" "memory");
    }

    far_jump_to(selector)
}

/// Load a segment into the stack segment register.
pub unsafe fn load_stack_segment(selector: SegmentSelector) {
    asm!("movw $0, %ss " :: "r"(selector.0) : "memory" : "volatile");
}

/// Load a segment into the data segment register.
pub unsafe fn load_data_segment(selector: SegmentSelector) {
    asm!("movw $0, %ds " :: "r"(selector.0) : "memory" : "volatile");
}

/// Load a segment into the extra segment register.
pub unsafe fn load_extra_segment(selector: SegmentSelector) {
    asm!("movw $0, %es " :: "r"(selector.0) : "memory" : "volatile");
}

/// Load a segment into the F segment register.
pub unsafe fn load_f_segment(selector: SegmentSelector) {
    asm!("movw $0, %fs " :: "r"(selector.0) : "memory" : "volatile");
}

/// Load a segment into the G segment register.
pub unsafe fn load_g_segment(selector: SegmentSelector) {
    asm!("movw $0, %gs " :: "r"(selector.0) : "memory" : "volatile");
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
        asm!("lgdt ($0)" :: "r" (gdt) : "memory");
    }
}

/// Load the local descriptor table into memory.
pub fn lldt<T>(ldt: &DescriptorTablePointer<T>) {
    unsafe {
        asm!("lldt ($0)" :: "r" (ldt) : "memory");
    }
}

/// Load the interrupt descriptor table into memory.
pub fn lidt<T>(idt: &DescriptorTablePointer<T>) {
    unsafe {
        asm!("lidt ($0)" :: "r" (idt) : "memory");
    }
}

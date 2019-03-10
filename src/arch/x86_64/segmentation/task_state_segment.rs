use super::SegmentSelector;

pub static mut TSS: TaskStateSegment = TaskStateSegment::new();

/// Represent the task state segment in memory
///
/// this structure is 104 bytes long and is used to handle interrupt like the stack overflow which
/// requires a new stack to be handled.
#[derive(Clone, Copy, Debug)]
#[repr(packed)]
pub struct TaskStateSegment {
    pub reserved: u32,
    /// spots to save stack pointers
    pub rsp: [u64; 3],
    pub reserved2: u64,
    /// table of stacks to use in case of a select few interrupt
    pub interrupt_stack_table: [u64; 7],
    pub reserved3: u64,
    pub reserved4: u16,
    pub iomap_base: u16,
}

impl TaskStateSegment {
    pub const fn new() -> TaskStateSegment {
        TaskStateSegment {
            reserved: 0,
            rsp: [0; 3],
            reserved2: 0,
            interrupt_stack_table: [0; 7],
            reserved3: 0,
            reserved4: 0,
            iomap_base: 0,
        }
    }
}

/// load the selector into the task register.
pub unsafe fn load_task_register(selector: SegmentSelector) {
    asm!("ltr $0" :: "r" (selector.0));
}

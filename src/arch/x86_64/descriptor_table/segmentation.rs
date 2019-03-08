use super::hardware::cpu;
use super::RingLevel;

/// Type associated with a memory segment entry in the GDT
#[allow(dead_code)]
#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum Type {
    Null = 0,
    KernelCode = 1,
    KernelData = 2,
    KernelThreadLocal = 3,
    UserCode = 4,
    UserData = 5,
    UserThreadLocal = 6,
    TaskState = 7,
    /// The task state segment must be 16 bytes long
    TaskStateHigh = 8,
}

/// Entry in the GDT.
///
/// For historic reason this structure is using a weird layout.
#[derive(Copy, Clone, Debug)]
#[repr(packed)]
pub struct Descriptor {
    limit_l: u16,
    address_l: u16,
    address_m: u8,
    access: u8,
    flags_limit_h: u8,
    address_h: u8,
}

impl Descriptor {
    pub const fn new(address: u32, limit: u32, access: Access, flags: Flag) -> Self {
        Descriptor {
            limit_l: limit as u16,
            address_l: address as u16,
            address_m: (address >> 16) as u8,
            access: access.0,
            flags_limit_h: flags.value() & 0xf0 | (limit >> 16) as u8 & 0x0f,
            address_h: (address >> 24) as u8,
        }
    }
}

/// Represent the access bits of the segment descriptor.
#[derive(Copy, Clone, Debug)]
pub struct Access(u8);

impl Access {
    pub const fn new(
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
pub struct Flag(pub cpu::Mode, pub Granularity);

impl Flag {
    pub const fn value(&self) -> u8 {
        ((self.0 as u8 & 1) << 1 | (self.0 as u8 & 2) >> 1) << 5 | (self.1 as u8) << 7
    }
}

/// Unit for the limit attribute of a segment.
/// * Byte mode uses 1 byte blocks
/// * Page mode uses 4kb blocks
#[allow(dead_code)]
#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum Granularity {
    Byte = 0,
    Page = 1,
}

/// Index of a segment in the GDT.
pub struct Selector(pub u16);

impl Selector {
    pub const fn new(index: Type, ring: RingLevel) -> Self {
        Selector(((index as u8) << 3 | ring as u8) as u16)
    }
}

/// Load a segment into the code segment register.
pub unsafe fn load_cs(selector: Selector) {
    // TODO
}

/// Load a segment into the stack segment register.
pub unsafe fn load_ss(selector: Selector) {
    asm!("movw $0, %ss " :: "r"(selector.0) : "memory" : "volatile");
}

/// Load a segment into the data segment register.
pub unsafe fn load_ds(selector: Selector) {
    asm!("movw $0, %ds " :: "r"(selector.0) : "memory" : "volatile");
}

/// Load a segment into the extra segment register.
pub unsafe fn load_es(selector: Selector) {
    asm!("movw $0, %es " :: "r"(selector.0) : "memory" : "volatile");
}

/// Load a segment into the F segment register.
pub unsafe fn load_fs(selector: Selector) {
    asm!("movw $0, %fs " :: "r"(selector.0) : "memory" : "volatile");
}

/// Load a segment into the G segment register.
pub unsafe fn load_gs(selector: Selector) {
    asm!("movw $0, %gs " :: "r"(selector.0) : "memory" : "volatile");
}

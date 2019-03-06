use super::hardware;
use core::mem::size_of;

/// Global Descriptor Table (GDT)
pub mod global;

/// Local Descriptor Table (LDT)
pub mod local;

/// Interrupt Descriptor Table (IDT)
pub mod interrupt;

/// # Descriptor Table pointer
///
/// Represent a 32 bit descriptor table into memory.
#[repr(C, packed)]
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

/// # Ring Level
///
/// Privilege level required to interact with a descriptor segment
#[allow(dead_code)]
#[derive(Copy, Clone, Debug)]
#[repr(u8)]
enum RingLevel {
    Zero = 0,
    One = 1,
    Two = 2,
    Three = 3,
}

/// # Load GDT
///
/// Load the global offset table into memory using the `lgdt` asm instruction.
pub fn lgdt<T>(gdt: &DescriptorTablePointer<T>) {
    unsafe {
        asm!("lgdt ($0)" :: "r" (gdt) : "memory");
    }
}

/// # Load LDT
///
/// Load the local descriptor table into memory using the `lldt` asm instruction.
pub fn lldt<T>(ldt: &DescriptorTablePointer<T>) {
    unsafe {
        asm!("lldt ($0)" :: "r" (ldt) : "memory");
    }
}

/// # Load IDT
///
/// Load the interrupt descriptor table into memory using the `lidt` asm instruction.
pub fn lidt<T>(idt: &DescriptorTablePointer<T>) {
    unsafe {
        asm!("lidt ($0)" :: "r" (idt) : "memory");
    }
}

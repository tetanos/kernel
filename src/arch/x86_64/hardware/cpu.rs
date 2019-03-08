/// Enumeration of the different mode the cpu can use.
///
/// This code will probably always be runing in long mode.
#[allow(dead_code)]
#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum Mode {
    /// 16 bits Real Mode
    Real = 0,
    /// 32 bits Protected Mode
    Protected = 1,
    /// 64 bits Long Mode
    Long = 2,
}

/// Representation of the cpu registers at a moment in time.
///
/// These values are not in sync with the actual registers.
#[derive(Copy, Clone, Debug)]
#[repr(packed)]
pub struct Registers {
    pub rax: usize,
    pub rcx: usize,
    pub rdx: usize,
    pub rdi: usize,
    pub rsi: usize,
    pub r8: usize,
    pub r9: usize,
    pub r10: usize,
    pub r11: usize,

    pub rbx: usize,
    pub rbp: usize,
    pub r12: usize,
    pub r13: usize,
    pub r14: usize,
    pub r15: usize,
}

impl Registers {
    /// Push all the registers onto the stack.
    ///
    /// > **Caution:** this method is very unsafe, you must call Registers::pop before calling any
    /// other methods or you will hit undefined opcodes.
    ///
    /// The idea is to line everything up to cast the address of the stack pointer as Self.
    #[inline(always)]
    pub unsafe fn push() {
        asm!(
        "push rax
         push rcx
         push rdx
         push rdi
         push rsi
         push r8
         push r9
         push r10
         push r11
         push rbx
         push rbp
         push r12
         push r13
         push r14
         push r15"
        : : : : "intel", "volatile");
    }

    /// This method will pop an item into every register.
    ///
    /// Should be called after Registers::push or it will definetly crash the entire kernel.
    #[inline(always)]
    pub unsafe fn pop() {
        asm!(
        "pop r15
         pop r14
         pop r13
         pop r12
         pop rbp
         pop rbx
         pop r11
         pop r10
         pop r9
         pop r8
         pop rsi
         pop rdi
         pop rdx
         pop rcx
         pop rax" : : : : "intel", "volatile");
    }
}

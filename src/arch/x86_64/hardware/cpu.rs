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
#[derive(Debug, Copy, Clone)]
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
        "push r15
         push r14
         push r13
         push r12
         push rbp
         push rbx
         push r11
         push r10
         push r9
         push r8
         push rsi
         push rdi
         push rdx
         push rcx
         push rax"
         : : : : "intel", "volatile");
    }

    /// This method will pop an item into every register.
    ///
    /// Should be called after Registers::push or it will definetly crash the entire kernel.
    #[inline(always)]
    pub unsafe fn pop() {
        asm!(
        "pop rax
         pop rcx
         pop rdx
         pop rdi
         pop rsi
         pop r8
         pop r9
         pop r10
         pop r11
         pop rbx
         pop rbp
         pop r12
         pop r13
         pop r14
         pop r15"
          : : : : "intel", "volatile");
    }
}

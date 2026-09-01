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

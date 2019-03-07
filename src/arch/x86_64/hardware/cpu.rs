use core::fmt;

#[allow(dead_code)]
#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum Mode {
    Real = 0,
    Protected = 1,
    Long = 2,
}

#[allow(dead_code)]
#[repr(packed)]
pub struct Regsiters {
    pub rax: usize,
    pub rbx: usize,
    pub rcx: usize,
    pub rdx: usize,
    pub rsi: usize,
    pub rdi: usize,
    pub rbp: usize,
    pub rsp: usize,
    pub r8: usize,
    pub r9: usize,
    pub r10: usize,
    pub r11: usize,
    pub r12: usize,
    pub r13: usize,
    pub r14: usize,
    pub r15: usize,
}

impl fmt::Debug for Registers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Registers {{ rax: {:>016x}, rbx: {:>016x}, rcx: {:>016x}, rdx: {:>016x}, rsi: {:>016x}, rdi: {:>016x}, rbp: {:>016x}, rsp: {:>016x}, r8: {:>016x}, r9: {:>016x}, r10: {:>016x}, r11: {:>016x}, r12: {:>016x}, r13: {:>016x}, r14: {:>016x}, r15: {:>016x} }}",
            self.rax,
            self.rbx,
            self.rcx,
            self.rdx,
            self.rsi,
            self.rdi,
            self.rbp,
            self.rsp,
            self.r8,
            self.r9,
            self.r10,
            self.r11,
            self.r12,
            self.r13,
            self.r14,
            self.r15
        )
        let mut s = f.debug_struct("Registers");
        s.field("rax", &Hex(self.rax));
        s.field("rbx", &Hex(self.rbx));
        s.field("rcx", &Hex(self.rcx));
        s.field("rdx", &Hex(self.rdx));
        s.field("rsi", &Hex(self.rsi));
        s.field("rdi", &Hex(self.rdi));
        s.field("rbp", &Hex(self.rbp));
        s.field("rsp", &Hex(self.rsp));
        s.field("r8", &Hex(self.r8));
        s.field("r9", &Hex(self.r9));
        s.field("r10", &Hex(self.r10));
        s.field("r11", &Hex(self.r11));
        s.field("r12", &Hex(self.r12));
        s.field("r13", &Hex(self.r13));
        s.field("r14", &Hex(self.r14));
        s.field("r15", &Hex(self.r15));
        s.finish()
    }
}

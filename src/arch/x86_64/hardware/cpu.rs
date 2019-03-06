#[allow(dead_code)]
#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum Mode {
    Real = 0,
    Protected = 1,
    Long = 2,
}

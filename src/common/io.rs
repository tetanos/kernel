use core::cmp::PartialEq;

pub trait IO {
    type Value: Copy + PartialEq;
    fn read(&self) -> Self::Value;
    fn write(&mut self, value: Self::Value);
}

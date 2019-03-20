use core::marker::PhantomData;

use crate::common::IO;

pub struct ProgrammedIO<T> {
    port: u16,
    value: PhantomData<T>,
}

impl<T> ProgrammedIO<T> {
    pub const fn new(port: u16) -> Self {
        ProgrammedIO::<T> {
            port: port,
            value: PhantomData,
        }
    }
}

impl IO for ProgrammedIO<u8> {
    type Value = u8;

    #[inline(always)]
    fn read(&self) -> u8 {
        let value: u8;
        unsafe {
            asm!("in $0, $1" : "={al}"(value) : "{dx}"(self.port) : "memory" : "intel", "volatile");
        }
        value
    }

    #[inline(always)]
    fn write(&mut self, value: u8) {
        unsafe {
            asm!("out $1, $0" : : "{al}"(value), "{dx}"(self.port) : "memory" : "intel", "volatile");
        }
    }
}

impl IO for ProgrammedIO<u16> {
    type Value = u16;

    #[inline(always)]
    fn read(&self) -> u16 {
        let value: u16;
        unsafe {
            asm!("in $0, $1" : "={al}"(value) : "{dx}"(self.port) : "memory" : "intel", "volatile");
        }
        value
    }

    #[inline(always)]
    fn write(&mut self, value: u16) {
        unsafe {
            asm!("out $1, $0" : : "{al}"(value), "{dx}"(self.port) : "memory" : "intel", "volatile");
        }
    }
}

impl IO for ProgrammedIO<u32> {
    type Value = u32;

    #[inline(always)]
    fn read(&self) -> u32 {
        let value: u32;
        unsafe {
            asm!("in $0, $1" : "={al}"(value) : "{dx}"(self.port) : "memory" : "intel", "volatile");
        }
        value
    }

    #[inline(always)]
    fn write(&mut self, value: u32) {
        unsafe {
            asm!("out $1, $0" : : "{al}"(value), "{dx}"(self.port) : "memory" : "intel", "volatile");
        }
    }
}

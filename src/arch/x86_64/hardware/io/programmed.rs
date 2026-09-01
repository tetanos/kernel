use core::arch::asm;
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
            asm!("in al, dx", out("al") value, in("dx") self.port);
        }
        value
    }

    #[inline(always)]
    fn write(&mut self, value: u8) {
        unsafe {
            asm!("out dx, al", in("al") value, in("dx") self.port);
        }
    }
}

impl IO for ProgrammedIO<u16> {
    type Value = u16;

    #[inline(always)]
    fn read(&self) -> u16 {
        let value: u16;
        unsafe {
            asm!("in ax, dx", out("ax") value, in("dx") self.port);
        }
        value
    }

    #[inline(always)]
    fn write(&mut self, value: u16) {
        unsafe {
            asm!("out dx, ax", in("ax") value, in("dx") self.port);
        }
    }
}

impl IO for ProgrammedIO<u32> {
    type Value = u32;

    #[inline(always)]
    fn read(&self) -> u32 {
        let value: u32;
        unsafe {
            asm!("in eax, dx", out("eax") value, in("dx") self.port);
        }
        value
    }

    #[inline(always)]
    fn write(&mut self, value: u32) {
        unsafe {
            asm!("out dx, eax", in("eax") value, in("dx") self.port);
        }
    }
}

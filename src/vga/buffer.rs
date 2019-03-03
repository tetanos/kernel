use volatile::Volatile;

use super::*;

#[repr(transparent)]
pub struct Buffer {
    pub chars: [[Volatile<StyledCharacter>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct StyledCharacter {
    pub character: u8,
    pub style: StyleByte,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct StyleByte(pub u8);

impl StyleByte {
    pub fn new(foreground: Color, background: Color) -> StyleByte {
        StyleByte((background as u8) << 4 | foreground as u8)
    }

    pub fn new_raw(value: u8) -> StyleByte {
        StyleByte(value)
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

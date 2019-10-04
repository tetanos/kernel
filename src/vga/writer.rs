use crate::vga::buffer::{Buffer, Color, StyleByte, StyledCharacter};
use crate::vga::{BUFFER_HEIGHT, BUFFER_WIDTH};
use core::fmt;

const VGA_ADDRESS: usize = 0xb8000;

pub struct Writer {
    cursor_x: usize,
    current_style: StyleByte,
    buffer: &'static mut Buffer,
}

impl Writer {
    pub fn new(foreground: Color, background: Color) -> Writer {
        Writer {
            cursor_x: 0,
            current_style: StyleByte::new(foreground, background),
            buffer: unsafe { &mut *(VGA_ADDRESS as *mut Buffer) },
        }
    }

    fn write(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                0x20...0x7e | b'\n' => self.write_byte(byte),
                0x08 => self.erase_byte(),
                _ => self.write_byte(0xfe),
            }
        }
    }

    fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.write_new_line(),
            byte => self.write_character(byte),
        }
    }

    fn erase_byte(&mut self) {
        if self.cursor_x <= 0 {
            return;
        }

        self.cursor_x -= 1;

        let screen_char = StyledCharacter {
            character: 0x20,
            style: self.current_style,
        };

        self.buffer.chars[BUFFER_HEIGHT - 1][self.cursor_x].write(screen_char);
    }

    fn write_character(&mut self, byte: u8) {
        if self.cursor_x >= BUFFER_WIDTH {
            self.write_new_line()
        }

        let screen_char = StyledCharacter {
            character: byte,
            style: self.current_style,
        };

        self.buffer.chars[BUFFER_HEIGHT - 1][self.cursor_x].write(screen_char);
        self.cursor_x += 1;
    }

    fn write_new_line(&mut self) {
        for y in 1..BUFFER_HEIGHT {
            for x in 0..BUFFER_WIDTH {
                let c = self.buffer.chars[y][x].read();
                self.buffer.chars[y - 1][x].write(c);
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.cursor_x = 0;
    }

    fn clear_row(&mut self, y: usize) {
        let blank = StyledCharacter {
            character: 0x20,
            style: self.current_style,
        };

        for x in 0..BUFFER_WIDTH {
            self.buffer.chars[y][x].write(blank);
        }
    }

    pub fn set_foreground_color(&mut self, color: Color) {
        self.current_style = StyleByte::new_raw((self.current_style.0 & 0xf0) | color as u8);
    }

    pub fn set_background_color(&mut self, color: Color) {
        self.current_style =
            StyleByte::new_raw((self.current_style.0 & 0xf) | ((color as u8) << 4));
    }

    pub fn rainbow_next(&mut self) {
        let u = (self.current_style.0 + 1) & 0x7 | 0x8;
        self.current_style = StyleByte::new_raw((self.current_style.0 & 0xf0) | u);
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write(s);
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use volatile::Volatile;

    fn construct_writer() -> Writer {
        use std::boxed::Box;

        let buffer = construct_buffer();
        Writer {
            cursor_x: 0,
            current_style: StyleByte::new(Color::Blue, Color::Magenta),
            buffer: Box::leak(Box::new(buffer)),
        }
    }

    fn construct_buffer() -> Buffer {
        use array_init::array_init;

        Buffer {
            chars: array_init(|_| array_init(|_| Volatile::new(empty_char()))),
        }
    }

    fn empty_char() -> StyledCharacter {
        StyledCharacter {
            character: b' ',
            style: StyleByte::new(Color::Green, Color::Brown),
        }
    }

    #[test]
    fn write_byte() {
        let mut writer = construct_writer();
        writer.write_byte(b'X');
        writer.write_byte(b'Y');

        for (i, row) in writer.buffer.chars.iter().enumerate() {
            for (j, screen_char) in row.iter().enumerate() {
                let screen_char = screen_char.read();
                if i == BUFFER_HEIGHT - 1 && j == 0 {
                    assert_eq!(screen_char.character, b'X');
                    assert_eq!(screen_char.style, writer.current_style);
                } else if i == BUFFER_HEIGHT - 1 && j == 1 {
                    assert_eq!(screen_char.character, b'Y');
                    assert_eq!(screen_char.style, writer.current_style);
                } else {
                    assert_eq!(screen_char, empty_char());
                }
            }
        }
    }
}

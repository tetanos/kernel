use core::fmt;

use super::*;

const VGA_ADDRESS: usize = 0xb8000;

pub struct Writer {
    cursor_x: usize,
    current_style: StyleByte,
    buffer: &'static mut buffer::Buffer,
}

impl Writer {
    pub fn new(foreground: Color, background: Color) -> Writer {
        Writer {
            cursor_x: 0,
            current_style: StyleByte::new(foreground, background),
            buffer: unsafe { &mut *(VGA_ADDRESS as *mut buffer::Buffer) },
        }
    }

    fn write(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                0x20...0x7e | b'\n' => self.write_byte(byte),
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
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write(s);
        Ok(())
    }
}

use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

const VGA_ADDRESS: usize = 0xb8000;

lazy_static! {
    pub static ref WRITER: Mutex<Writer> =
        Mutex::new(Writer::new(Color::LightGray, Color::Black, true, false));
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}

#[allow(dead_code)]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Black = 0x0,
    Blue = 0x1,
    Green = 0x2,
    Cyan = 0x3,
    Red = 0x4,
    Magenta = 0x5,
    Brown = 0x6,
    LightGray = 0x7,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct StyleByte(u8);

impl StyleByte {
    fn new(foreground: Color, background: Color, bright: bool, blink: bool) -> StyleByte {
        StyleByte(
            (blink as u8) << 7 | (background as u8) << 4 | (bright as u8) << 3 | (foreground as u8),
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii: u8,
    style: StyleByte,
}

#[repr(transparent)]
struct Buffer {
    chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    cursor_x: usize,
    current_style: StyleByte,
    buffer: &'static mut Buffer,
}

impl Writer {
    pub fn new(foreground: Color, background: Color, bright: bool, blink: bool) -> Writer {
        Writer {
            cursor_x: 0,
            current_style: StyleByte::new(foreground, background, bright, blink),
            buffer: unsafe { &mut *(VGA_ADDRESS as *mut Buffer) },
        }
    }

    pub fn write(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                0x20...0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe),
            }
        }
    }

    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.write_new_line(),
            byte => {
                if self.cursor_x >= BUFFER_WIDTH {
                    self.write_new_line()
                }

                let screen_char = ScreenChar {
                    ascii: byte,
                    style: self.current_style,
                };

                self.buffer.chars[BUFFER_HEIGHT - 1][self.cursor_x] = screen_char;
                self.cursor_x += 1;
            }
        }
    }

    pub fn write_new_line(&mut self) {
        for y in 1..BUFFER_HEIGHT {
            for x in 0..BUFFER_WIDTH {
                let c = self.buffer.chars[y][x];
                self.buffer.chars[y - 1][x] = c;
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.cursor_x = 0;
    }

    fn clear_row(&mut self, y: usize) {
        let blank = ScreenChar {
            ascii: 0x20,
            style: self.current_style,
        };

        for x in 0..BUFFER_WIDTH {
            self.buffer.chars[y][x] = blank;
        }
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write(s);
        Ok(())
    }
}

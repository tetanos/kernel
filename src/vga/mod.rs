pub mod buffer;
pub mod term;
pub mod writer;

use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;

use buffer::Color;
use writer::Writer;

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer::new(Color::White, Color::Black));
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;
    interrupts::without_interrupts(|| {
        WRITER.lock().write_fmt(args).unwrap();
    });
}

pub fn ferris_say(s: &str) {
    let ferris = r#"
      \
       \
          _~^~^~_
      \) /  o o  \ (/
        '_   -   _'
        / '-----' \
"#;

    for c in s.chars() {
        if c != ' ' {
            rainbow_next();
        }
        print!("{}", c);
    }

    set_foreground_color(Color::Red);
    println!("{}", ferris);
    set_foreground_color(Color::White);
}

pub fn rainbow_next() {
    use x86_64::instructions::interrupts;
    interrupts::without_interrupts(|| {
        WRITER.lock().rainbow_next();
    });
}

#[allow(dead_code)]
pub fn set_foreground_color(color: Color) {
    use x86_64::instructions::interrupts;
    interrupts::without_interrupts(|| {
        WRITER.lock().set_foreground_color(color);
    });
}

#[allow(dead_code)]
pub fn set_background_color(color: Color) {
    use x86_64::instructions::interrupts;
    interrupts::without_interrupts(|| {
        WRITER.lock().set_background_color(color);
    });
}

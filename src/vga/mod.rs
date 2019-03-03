pub mod buffer;
pub mod writer;

use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;

use buffer::*;
use writer::*;

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer::new(Color::LightGray, Color::Black));
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
    WRITER.lock().write_fmt(args).unwrap();
}

pub fn ferris_say(something: &str) {
    let ferris = r#"
      \
       \
          _~^~^~_
      \) /  o o  \ (/
        '_   -   _'
        / '-----' \
"#;

    println!("{}{}", something, ferris);
}

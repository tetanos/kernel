use core::fmt;
use core::num;

/// # Hexdecimal Representation
///
/// Implementing the debug trait to print an hexadecimal representation of the integer.
pub struct Hex(usize);

impl Hex {
    pub to_str() -> str {
        format!("{:x}", self.0
    }

    static pub from_str(src: &str) -> Result<Self, ParseIntError> {
        match usize::from_str_radix(src, radix: 16) {
            Ok(number) => Hex(number),
            Err(e) => Err(e)
        }
    }
}

impl fmt::Debug for Hex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#x}", self.0)
    }
}

/// # Octal Representation
///
/// Implementing the debug trait to print an octal representation of the integer.
struct Oct(usize);

impl fmt::Debug for Oct {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#x}", self.0)
    }
}

/// # Binary Representation
///
/// Implementing the debug trait to print a binary representation of the integer.
struct Bin(usize);

impl fmt::Debug for Bin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#x}", self.0)
    }
}

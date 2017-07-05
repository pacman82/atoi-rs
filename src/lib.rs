//! A crate for parsing integers directly form ASCII (`[u8]`) without encoding them into utf8
//! first. The name is inspired by the famous C function.
//!
//! Using `str::from_utf8` and `str::parse`
//! is likely to be more idiomatic. Use this crate if you want to avoid decoding utf8 (e.g. for
//! performance reasons), but stick to safe code where using `str::from_ut8_unchecked` is not an
//! option.

extern crate num_traits;
use num_traits::{Zero, One, Signed};
use std::ops::{AddAssign, MulAssign};

/// Types implementing this trait can be parsed from a positional numeral system with radix 10
pub trait FromRadix10: Sized {
    /// Parses an integer from a slice.
    ///
    /// # Example
    ///
    /// ```
    /// use atoi::FromRadix10;
    /// // Parsing to digits from a slice
    /// assert_eq!((42,2), u32::from_radix_10(b"42"));
    /// // Additional bytes after the number are ignored
    /// assert_eq!((42,2), u32::from_radix_10(b"42 is the answer to life, the universe and everything"));
    /// // (0,0) is returned if the slice does not start with a digit
    /// assert_eq!((0,0), u32::from_radix_10(b"Sadly we do not know the question"));
    /// // While signed integer types are supported...
    /// assert_eq!((42,2), i32::from_radix_10(b"42"));
    /// // Signs are not allowed (even for signed integer types)
    /// assert_eq!((0,0), i32::from_radix_10(b"-42"));
    /// // Leading zeros are allowed
    /// assert_eq!((42,4), u32::from_radix_10(b"0042"));
    /// ```
    ///
    /// # Return
    /// Returns a tuple with two numbers. The first is the integer parsed or zero, the second is the
    /// index of the byte right after the parsed number. If the second element is zero the slice
    /// did not start with an ASCII digit.
    fn from_radix_10(&[u8]) -> (Self, usize);
}

/// Parses an integer from a slice.
///
/// Contrary to its 'C' counterpart atoi is generic and will require a type argument if the type
/// inference can not determine its result.
///
/// # Example
///
/// ```
/// use atoi::atoi;
/// // Parsing to digits from a slice
/// assert_eq!(Some(42), atoi::<u32>(b"42"));
/// // Additional bytes after the number are ignored. If you want to know how many bytes were used
/// // to parse the number use `FromRadix10::from_radix_10`.
/// assert_eq!(Some(42), atoi::<u32>(b"42 is the answer to life, the universe and everything"));
/// // `None` is returned if the slice does not start with a digit
/// assert_eq!(None, atoi::<u32>(b"Sadly we do not know the question"));
/// // While signed integer types are supported...
/// assert_eq!(Some(42), atoi::<i32>(b"42"));
/// // ... signs currently are not (subject to change in future versions)
/// assert_eq!(None, atoi::<i32>(b"-42"));
/// // Leading zeros are allowed
/// assert_eq!(Some(42), atoi::<u32>(b"0042"));
/// ```
///
/// # Return
/// Returns a a number if the slice started with a number, otherwise `None` is returned.
pub fn atoi<I>(text: &[u8]) -> Option<I>
    where I: FromRadix10
{
    match I::from_radix_10(text) {
        (_, 0) => None,
        (n, _) => Some(n),
    }
}

/// Converts an ascii character to digit
///
/// # Example
///
/// ```
/// use atoi::ascii_to_digit;
/// assert_eq!(Some(5), ascii_to_digit(b'5'));
/// assert_eq!(None, ascii_to_digit::<u32>(b'x'));
/// ```
pub fn ascii_to_digit<I>(character: u8) -> Option<I>
    where I: Zero + One
{
    match character {
        b'0' => Some(nth(0)),
        b'1' => Some(nth(1)),
        b'2' => Some(nth(2)),
        b'3' => Some(nth(3)),
        b'4' => Some(nth(4)),
        b'5' => Some(nth(5)),
        b'6' => Some(nth(6)),
        b'7' => Some(nth(7)),
        b'8' => Some(nth(8)),
        b'9' => Some(nth(9)),
        _ => None,
    }
}

impl<I> FromRadix10 for I
    where I: Zero + One + AddAssign + MulAssign
{
    fn from_radix_10(text: &[u8]) -> (Self, usize) {
        let mut index = 0;
        let mut number = I::zero();
        while index != text.len() {
            if let Some(digit) = ascii_to_digit(text[index]) {
                number *= nth(10);
                number += digit;
                index += 1;
            } else {
                break;
            }
        }
        (number, index)
    }
}

/// Representation of a numerical sign
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sign {
    Plus,
    Minus,
}

impl Sign {
    /// Converts an ascii character to digit
    ///
    /// # Example
    ///
    /// ```
    /// use atoi::Sign;
    /// assert_eq!(Some(Sign::Plus), Sign::try_from(b'+'));
    /// assert_eq!(Some(Sign::Minus), Sign::try_from(b'-'));
    /// assert_eq!(None, Sign::try_from(b'1'));
    /// ```
    pub fn try_from(byte: u8) -> Option<Sign> {
        match byte {
            b'+' => Some(Sign::Plus),
            b'-' => Some(Sign::Minus),
            _ => None,
        }
    }

    /// Returns either `+1` or `-1`
    pub fn signum<I>(self) -> I
        where I: Signed
    {
        match self {
            Sign::Plus => I::one(),
            Sign::Minus => -I::one(),
        }
    }
}

// At least for primitive types this function does not incur runtime costs, since it is only called
// with constants
fn nth<I>(n: usize) -> I
    where I: Zero + One
{
    let mut i = I::zero();
    for _ in 0..n {
        i = i + I::one();
    }
    i
}

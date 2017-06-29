//! A crate for parsing integers directly form ASCII (`[u8]`) without encoding them into utf8
//! first. The name is inspired by the famous C function.
//!
//! Using `str::from_utf8` and `str::parse`
//! is likely to be more idiomatic. Use this crate if you want to avoid decoding utf8 (e.g. for
//! performance reasons), but stick to safe code where using `str::from_ut8_unchecked` is not an
//! option.

extern crate num_traits;
use num_traits::{Zero, One};
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
        b'0' => Some(I::zero()),
        b'1' => Some(I::one()),
        b'2' => Some(I::two()),
        b'3' => Some(I::three()),
        b'4' => Some(I::four()),
        b'5' => Some(I::five()),
        b'6' => Some(I::six()),
        b'7' => Some(I::seven()),
        b'8' => Some(I::eight()),
        b'9' => Some(I::nine()),
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
                number *= I::ten();
                number += digit;
                index += 1;
            } else {
                break;
            }
        }
        (number, index)
    }
}

trait ZeroToTen: Zero + One {
    fn two() -> Self;
    fn three() -> Self;
    fn four() -> Self;
    fn five() -> Self;
    fn six() -> Self;
    fn seven() -> Self;
    fn eight() -> Self;
    fn nine() -> Self;
    fn ten() -> Self;
}

impl<T> ZeroToTen for T
    where T: Zero + One
{
    fn two() -> T {
        T::one() + T::one()
    }

    fn three() -> T {
        T::two() + T::one()
    }

    fn four() -> T {
        T::three() + T::one()
    }

    fn five() -> T {
        T::four() + T::one()
    }

    fn six() -> T {
        T::five() + T::one()
    }

    fn seven() -> T {
        T::six() + T::one()
    }

    fn eight() -> T {
        T::seven() + T::one()
    }

    fn nine() -> T {
        T::eight() + T::one()
    }

    fn ten() -> T {
        T::nine() + T::one()
    }
}

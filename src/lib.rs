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
/// assert_eq!((42,2), atoi::<u32>(b"42"));
/// // Additional bytes after the number are ignored
/// assert_eq!((42,2), atoi::<u32>(b"42 is the answer to life, the universe and everything"));
/// // (0,0) is returned if the slice does not start with a digit
/// assert_eq!((0,0), atoi::<u32>(b"Sadly we do not know the question"));
/// // While signed integer types are supported...
/// assert_eq!((42,2), atoi::<i32>(b"42"));
/// // ... signs currently are not (subject to change in future versions)
/// assert_eq!((0,0), atoi::<i32>(b"-42"));
/// ```
///
/// # Return
/// Returns a tuple with two numbers. The first is the integer parsed or zero, the second is the
/// index of the byte right after the parsed number. If the second element is zero the slice
/// did not start with an ASCII digit.
pub fn atoi<I>(text: &[u8]) -> (I, usize)
    where I: Zero + One + AddAssign + MulAssign
{
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

trait Inductive: Zero + One {
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

impl<T> Inductive for T
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

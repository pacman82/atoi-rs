//! A crate for parsing integers directly form ASCII (`[u8]`) without encoding them into utf8
//! first. The name is inspired by the famous C function.
//!
//! Using `str::from_utf8` and `str::parse`
//! is likely to be more idiomatic. Use this crate if you want to avoid decoding bytes into utf8
//! (e.g. for performance reasons).
//!
//! Note that if you want to know how much of the input has been used, you can use the
//! `FromRadix10` trait, for example:
//!
//! ```rust
//! use atoi::FromRadix10;
//!
//! /// Return the parsed integer and remaining slice if successful.
//! fn atoi_with_rest<I: FromRadix10>(text: &[u8]) -> ((&[u8], Option<I>)) {
//!     match I::from_radix_10(text) {
//!         (_, 0) => (text, None),
//!         (n, used) => (&text[used..], Some(n)),
//!     }
//! }
//! ```

use num_traits::{One, Signed, Zero, Bounded, ops::checked::{CheckedMul, CheckedAdd}};
use std::{ops::{AddAssign, MulAssign, DivAssign}, cmp::min};

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
/// // By default this will panic in debug or overflow in release builds.
/// // assert_eq!(Some(0), atoi::<u8>(b"256")).
/// // use the e.g. the `checked` crate to handle overflows gracefully
/// ```
///
/// # Return
///
/// Returns a a number if the slice started with a number, otherwise `None` is returned.
pub fn atoi<I>(text: &[u8]) -> Option<I>
where
    I: FromRadix10Checked,
{
    match I::from_radix_10_checked(text) {
        (_, 0) | (None, _) => None,
        (Some(n), _) => Some(n),
    }
}


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
    ///
    /// Returns a tuple with two numbers. The first is the integer parsed or zero, the second is the
    /// index of the byte right after the parsed number. If the second element is zero the slice
    /// did not start with an ASCII digit.
    fn from_radix_10(_: &[u8]) -> (Self, usize);
}

/// Types implementing this trait can be parsed from a positional numeral system with radix 10.
/// Acts much like `FromRadix10`, but performs additional checks for overflows.
pub trait FromRadix10Checked: FromRadix10 {
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
    ///
    /// Returns a tuple with two numbers. The first is the integer parsed or zero if no digit has
    /// been found. None, if there were too many, or too high dighits and the parsing overflowed.
    /// The second is the index of the byte right after the parsed number. If the second element is
    /// zero the slice did not start with an ASCII digit.
    fn from_radix_10_checked(_: &[u8]) -> (Option<Self>, usize);
}

/// A bounded integer, whose representation can overflow and therfore can only store a maximum
/// number of digits
pub trait MaxNumDigits {
    /// Given a representation with a radix charactar I, what is the maximum number of digits we can
    /// parse without the integer overflowing for sure?
    fn max_num_digits(radix: Self) -> usize;
}

impl<I> MaxNumDigits for I where I: Bounded + Zero + DivAssign + Ord +  Copy{
    /// Returns the maximum number of digits a representation of `I` can have depending on `radix`.
    fn max_num_digits(radix: I) -> usize {
        let mut max = I::max_value();
        let mut d = 0;
        while max > I::zero() {
            d += 1;
            max /= radix;
        }
        d
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
where
    I: Zero + One,
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
where
    I: Zero + One + AddAssign + MulAssign,
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

impl<I> FromRadix10Checked for I
where
    I: Zero + One + FromRadix10 + CheckedMul + CheckedAdd + MaxNumDigits,
{
    fn from_radix_10_checked(text: &[u8]) -> (Option<I>, usize) {
        let max_safe_digits = I::max_num_digits(nth(10)) - 1;
        let (number, mut index) = I::from_radix_10(&text[..min(text.len(),max_safe_digits)]);
        let mut number = Some(number);
        // We parsed the digits, which do not need checking now lets see the next one:
        while index != text.len(){
            if let Some(digit) = ascii_to_digit(text[index]) {
                number = number.and_then(|n| n.checked_mul(&nth(10)));
                number = number.and_then(|n| n.checked_add(&digit));
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
    /// Trys to convert an ascii character into a `Sign`
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
    where
        I: Signed,
    {
        match self {
            Sign::Plus => I::one(),
            Sign::Minus => -I::one(),
        }
    }
}

// At least for primitive types this function does not incur runtime costs, since it is only called
// with constants
fn nth<I>(n: u8) -> I
where
    I: Zero + One,
{
    let mut i = I::zero();
    for _ in 0..n {
        i = i + I::one();
    }
    i
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn max_digits(){
        assert_eq!(10, i32::max_num_digits(10));
        assert_eq!(10, u32::max_num_digits(10));
        assert_eq!(19, i64::max_num_digits(10));
        assert_eq!(20, u64::max_num_digits(10));
    }

    #[test]
    fn checked_parsing() {
        assert_eq!((Some(255), 3), u8::from_radix_10_checked(b"255"));
        assert_eq!((None, 3), u8::from_radix_10_checked(b"256"));
        assert_eq!((None, 4), u8::from_radix_10_checked(b"1000"));
        assert_eq!((Some(25), 2), u8::from_radix_10_checked(b"25"));
        assert_eq!((Some(25), 2), u8::from_radix_10_checked(b"25Blub"));
    }
}

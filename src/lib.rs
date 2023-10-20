//! A crate for parsing integers directly from ASCII (`[u8]`) without encoding them into utf8
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
#![cfg_attr(not(std), no_std)]

use num_traits::Signed;

mod builtin;
mod integer;

pub use integer::Integer;

/// Parses an integer from a slice.
///
/// Contrary to its 'C' counterpart atoi is generic and will require a type argument if the type
/// inference can not determine its result. It will also check for overflow / underflow and allow
/// for Signs.
///
/// Use [`FromRadix10`] or [`FromRadix10Checked`] directly if you do not want to allow signs. Use
/// [`FromRadix10`] or [`FromRadix10Signed`] if you want to opt out overflow / underflow checking.
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
/// // Signs are allowed.
/// assert_eq!(Some(-42), atoi::<i32>(b"-42"));
/// // Leading zeros are allowed
/// assert_eq!(Some(42), atoi::<u32>(b"0042"));
/// // Overflows will return `None`
/// assert_eq!(None, atoi::<u8>(b"256"));
/// ```
///
/// # Return
///
/// Returns a a number if the slice started with a number, otherwise `None` is returned.
pub fn atoi<I>(text: &[u8]) -> Option<I>
where
    I: FromRadix10SignedChecked,
{
    match I::from_radix_10_signed_checked(text) {
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
pub trait FromRadix10Checked: Sized {
    /// Parses an integer from a slice.
    ///
    /// # Example
    ///
    /// ```
    /// use atoi::FromRadix10Checked;
    /// // Parsing to digits from a slice
    /// assert_eq!((Some(42),2), u32::from_radix_10_checked(b"42"));
    /// // Additional bytes after the number are ignored
    /// assert_eq!((Some(42),2), u32::from_radix_10_checked(b"42 is the answer to life, the universe and everything"));
    /// // (0,0) is returned if the slice does not start with a digit
    /// assert_eq!((Some(0),0), u32::from_radix_10_checked(b"Sadly we do not know the question"));
    /// // While signed integer types are supported...
    /// assert_eq!((Some(42),2), i32::from_radix_10_checked(b"42"));
    /// // Signs are not allowed (even for signed integer types)
    /// assert_eq!((Some(0),0), i32::from_radix_10_checked(b"-42"));
    /// // Leading zeros are allowed
    /// assert_eq!((Some(42),4), u32::from_radix_10_checked(b"0042"));
    /// // Overflow is indicated by `None`
    /// assert_eq!((None, 3), u8::from_radix_10_checked(b"256"));
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

/// Types implementing this trait can be parsed from a positional numeral system with radix 16
pub trait FromRadix16: Sized {
    /// Parses an integer from a slice.
    ///
    /// # Example
    ///
    /// ```
    /// use atoi::FromRadix16;
    /// // Parsing to digits from a slice
    /// assert_eq!((42,2), u32::from_radix_16(b"2a"));
    /// // Additional bytes after the number are ignored
    /// assert_eq!((42,2), u32::from_radix_16(b"2a is the answer to life, the universe and everything"));
    /// // (0,0) is returned if the slice does not start with a digit
    /// assert_eq!((0,0), u32::from_radix_16(b"Sadly we do not know the question"));
    /// // While signed integer types are supported...
    /// assert_eq!((42,2), i32::from_radix_16(b"2a"));
    /// // Signs are not allowed (even for signed integer types)
    /// assert_eq!((0,0), i32::from_radix_16(b"-2a"));
    /// // Leading zeros are allowed
    /// assert_eq!((42,4), u32::from_radix_16(b"002a"));
    /// // so are uppercase letters
    /// assert_eq!((42,4), u32::from_radix_16(b"002A"));
    /// ```
    ///
    /// # Return
    ///
    /// Returns a tuple with two numbers. The first is the integer parsed or zero, the second is the
    /// index of the byte right after the parsed number. If the second element is zero the slice
    /// did not start with an ASCII digit.
    fn from_radix_16(_: &[u8]) -> (Self, usize);
}

/// Types implementing this trait can be parsed from a positional numeral system with radix 16.
/// Acts much like [`FromRadix16`], but performs additional checks for overflows.
pub trait FromRadix16Checked: Sized {
    /// Parses an integer from a slice.
    ///
    /// # Example
    ///
    /// ```
    /// use atoi::FromRadix16Checked;
    /// // Parsing to digits from a slice
    /// assert_eq!((Some(42),2), u32::from_radix_16_checked(b"2a"));
    /// // Additional bytes after the number are ignored
    /// assert_eq!((Some(42),2), u32::from_radix_16_checked(b"2a is the answer to life, the \
    /// universe and everything"));
    /// // (0,0) is returned if the slice does not start with a digit
    /// assert_eq!((Some(0),0), u32::from_radix_16_checked(b"Sadly we do not know the question"));
    /// // While signed integer types are supported...
    /// assert_eq!((Some(42),2), i32::from_radix_16_checked(b"2a"));
    /// // Signs are not allowed (even for signed integer types)
    /// assert_eq!((Some(0),0), i32::from_radix_16_checked(b"-2a"));
    /// // Leading zeros are allowed
    /// assert_eq!((Some(42),4), u32::from_radix_16_checked(b"002a"));
    /// // So are uppercase letters
    /// assert_eq!((Some(42),2), u32::from_radix_16_checked(b"2A"))
    /// ```
    ///
    /// # Return
    ///
    /// Returns a tuple with two numbers. The first is the integer parsed or zero if no digit has
    /// been found. None, if there were too many, or too high dighits and the parsing overflowed.
    /// The second is the index of the byte right after the parsed number. If the second element is
    /// zero the slice did not start with an ASCII digit.
    fn from_radix_16_checked(_: &[u8]) -> (Option<Self>, usize);
}

/// Types implementing this trait can be parsed from a positional numeral system with radix 10. This
/// trait allows for an additional sign character (`+` or `-`) in front of the actual number in
/// order, to allow for parsing negative values.
pub trait FromRadix10Signed: Sized {
    /// Parses an integer from a slice.
    ///
    /// # Example
    ///
    /// ```
    /// use atoi::FromRadix10Signed;
    /// // Parsing to digits from a slice
    /// assert_eq!((42,2), i32::from_radix_10_signed(b"42"));
    /// // Additional bytes after the number are ignored
    /// assert_eq!((42,2), i32::from_radix_10_signed(b"42 is the answer to life, the universe and everything"));
    /// // (0,0) is returned if the slice does not start with a digit
    /// assert_eq!((0,0), i32::from_radix_10_signed(b"Sadly we do not know the question"));
    /// // Signs are allowed
    /// assert_eq!((-42,3), i32::from_radix_10_signed(b"-42"));
    /// // Signs are allowed
    /// assert_eq!((42,3), i32::from_radix_10_signed(b"+42"));
    /// // Even on unsigned types.
    /// assert_eq!((0,2), u32::from_radix_10_signed(b"-0"));
    /// // Leading zeros are allowed
    /// assert_eq!((42,4), i32::from_radix_10_signed(b"0042"));
    /// ```
    ///
    /// # Return
    ///
    /// Returns a tuple with two numbers. The first is the integer parsed or zero, the second is the
    /// index of the byte right after the parsed number. If the second element is zero the slice
    /// did not start with an ASCII digit.
    fn from_radix_10_signed(_: &[u8]) -> (Self, usize);
}

/// Types implementing this trait can be parsed from a positional numeral system with radix 10.
/// Acts much like `FromRadix10Signed`, but performs additional checks for overflows.
pub trait FromRadix10SignedChecked: FromRadix10Signed {
    /// Parses an integer from a slice.
    ///
    /// # Example
    ///
    /// ```
    /// use atoi::FromRadix10SignedChecked;
    /// // Parsing to digits from a slice
    /// assert_eq!((Some(42),2), u32::from_radix_10_signed_checked(b"42"));
    /// // Additional bytes after the number are ignored
    /// assert_eq!((Some(42),2), u32::from_radix_10_signed_checked(b"42 is the answer to life, the universe and everything"));
    /// // (0,0) is returned if the slice does not start with a digit
    /// assert_eq!((Some(0),0), u32::from_radix_10_signed_checked(b"Sadly we do not know the question"));
    /// // While signed integer types are supported...
    /// assert_eq!((Some(42),2), i32::from_radix_10_signed_checked(b"42"));
    /// // Signs are allowed
    /// assert_eq!((Some(-42),3), i32::from_radix_10_signed_checked(b"-42"));
    /// // -0 is ok, even for an unsigned type
    /// assert_eq!((Some(0),2), u32::from_radix_10_signed_checked(b"-0"));
    /// // -1 is an Underflow
    /// assert_eq!((None,2), u32::from_radix_10_signed_checked(b"-1"));
    /// // Negative values for unsigned types are handled as `None`.
    /// assert_eq!((None,3), u32::from_radix_10_signed_checked(b"-42"));
    /// // Leading zeros are allowed
    /// assert_eq!((Some(42),4), u32::from_radix_10_signed_checked(b"0042"));
    /// // Overflow is indicated by `None`
    /// assert_eq!((None, 3), u8::from_radix_10_signed_checked(b"256"));
    /// assert_eq!((None, 4), i8::from_radix_10_signed_checked(b"+128"));
    /// assert_eq!((None, 4), i8::from_radix_10_signed_checked(b"-129"));
    /// ```
    ///
    /// # Return
    ///
    /// Returns a tuple with two numbers. The first is the integer parsed or zero if no digit has
    /// been found. None, if there were too many, or too high dighits and the parsing overflowed.
    /// The second is the index of the byte right after the parsed number. If the second element is
    /// zero the slice did not start with an ASCII digit.
    fn from_radix_10_signed_checked(_: &[u8]) -> (Option<Self>, usize);
}

/// A bounded integer, whose representation can overflow and therefore can only store a maximum
/// number of digits
pub trait MaxNumDigits {
    /// Given a representation with a radix character I, what is the maximum number of digits we can
    /// parse without the integer overflowing for sure?
    fn max_num_digits(radix: Self) -> usize;

    /// Returns the maximum number of digits a negative representation of `I` can have depending on
    /// `radix`.
    fn max_num_digits_negative(radix: Self) -> usize;
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

/// Construct an instance of a numerical type using the byte representation of a radix 10 digit,
/// e.g. b'7' -> 7.
pub trait FromDigit: Sized {
    /// Convert ASCII digit (e.g. b'7) into numeric representation (`7`). `None` if the character
    /// given does not represent a digit in ASCII.
    ///
    /// # Example
    ///
    /// ```
    /// use atoi::FromDigit;
    /// assert_eq!(Some(5), u32::from_digit(b'5'));
    /// assert_eq!(None, u32::from_digit(b'x'));
    /// ```
    fn from_digit(digit: u8) -> Option<Self>;
}

trait FromHexDigit: Sized {
    fn from_hex_digit(digit: u8) -> Option<Self>;
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
pub fn ascii_to_digit<I>(digit: u8) -> Option<I>
where
    I: FromDigit,
{
    I::from_digit(digit)
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn max_digits() {
        assert_eq!(10, i32::max_num_digits(10));
        assert_eq!(10, u32::max_num_digits(10));
        assert_eq!(19, i64::max_num_digits(10));
        assert_eq!(20, u64::max_num_digits(10));
        assert_eq!(3, u8::max_num_digits(10));
        assert_eq!(3, i8::max_num_digits(10));
    }

    #[test]
    fn max_digits_negative() {
        assert_eq!(10, i32::max_num_digits_negative(10));
        assert_eq!(0, u32::max_num_digits_negative(10));
        assert_eq!(19, i64::max_num_digits_negative(10));
        assert_eq!(0, u64::max_num_digits_negative(10));
        assert_eq!(0, u8::max_num_digits_negative(10));
        assert_eq!(3, i8::max_num_digits_negative(10));
    }

    #[test]
    fn checked_parsing() {
        assert_eq!((Some(255), 3), u8::from_radix_10_checked(b"255"));
        assert_eq!((None, 3), u8::from_radix_10_checked(b"256"));
        assert_eq!((None, 4), u8::from_radix_10_checked(b"1000"));
        assert_eq!((Some(25), 2), u8::from_radix_10_checked(b"25"));
        assert_eq!((Some(25), 2), u8::from_radix_10_checked(b"25Blub"));
    }

    #[test]
    fn checked_parsing_radix_16() {
        assert_eq!((Some(255), 2), u8::from_radix_16_checked(b"FF"));
        assert_eq!((None, 3), u8::from_radix_16_checked(b"100"));
        assert_eq!((None, 4), u8::from_radix_16_checked(b"1000"));
        assert_eq!((Some(25), 2), u8::from_radix_16_checked(b"19"));
        assert_eq!((Some(25), 2), u8::from_radix_16_checked(b"19!Blub"));
    }
}

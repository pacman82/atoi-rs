use core::{ops::{AddAssign, MulAssign, SubAssign, DivAssign}, cmp::{max, min}};

use num_traits::{Zero, One, CheckedAdd, CheckedSub, CheckedMul, Bounded};

use crate::{FromRadix10, FromRadix10Signed, Sign, FromRadix10SignedChecked, MaxNumDigits, FromRadix10Checked, FromRadix16, FromRadix16Checked};

/// Wrapper which implements the traits [`crate::FromRadix10`], [`crate::FromRadix10Checked`],
/// [`crate::FromRadix16`] and [`crate::FromRadix16Checked`] for any inductive type. I.e. a type
/// implementing [`One`], [`Zero`], `+=` and `*=`.
pub struct Integer<I>(pub I);

impl<I> FromRadix10 for Integer<I>
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
        (Integer(number), index)
    }
}

impl<I> FromRadix10Signed for Integer<I>
where
    I: Zero + One + AddAssign + SubAssign + MulAssign,
{
    fn from_radix_10_signed(text: &[u8]) -> (Self, usize) {
        let mut index;
        let mut number = I::zero();

        let (sign, offset) = text
            .first()
            .and_then(|&byte| Sign::try_from(byte))
            .map(|sign| (sign, 1))
            .unwrap_or((Sign::Plus, 0));

        index = offset;

        // Having two dedicated loops for both the negative and the nonnegative case is rather
        // verbose, yet performed up to 40% better then a more terse single loop with
        // `number += digit * signum`.

        match sign {
            Sign::Plus => {
                while index != text.len() {
                    if let Some(digit) = ascii_to_digit::<I>(text[index]) {
                        number *= nth(10);
                        number += digit;
                        index += 1;
                    } else {
                        break;
                    }
                }
            }
            Sign::Minus => {
                while index != text.len() {
                    if let Some(digit) = ascii_to_digit::<I>(text[index]) {
                        number *= nth(10);
                        number -= digit;
                        index += 1;
                    } else {
                        break;
                    }
                }
            }
        }

        (Integer(number), index)
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

impl<I> FromRadix10SignedChecked for Integer<I>
where
    I: Zero
        + One
        + AddAssign
        + MulAssign
        + SubAssign
        + CheckedAdd
        + CheckedSub
        + CheckedMul
        + MaxNumDigits,
{
    fn from_radix_10_signed_checked(text: &[u8]) -> (Option<Self>, usize) {
        let mut index;
        let mut number = I::zero();

        let (sign, offset) = text
            .first()
            .and_then(|&byte| Sign::try_from(byte))
            .map(|sign| (sign, 1))
            .unwrap_or((Sign::Plus, 0));

        index = offset;

        // Having two dedicated loops for both the negative and the nonnegative case is rather
        // verbose, yet performed up to 40% better then a more terse single loop with
        // `number += digit * signum`.

        match sign {
            Sign::Plus => {
                let max_safe_digits = max(1, I::max_num_digits(nth(10))) - 1;
                let max_safe_index = min(text.len(), max_safe_digits + offset);
                while index != max_safe_index {
                    if let Some(digit) = ascii_to_digit::<I>(text[index]) {
                        number *= nth(10);
                        number += digit;
                        index += 1;
                    } else {
                        break;
                    }
                }
                // We parsed the digits, which do not need checking now lets see the next one:
                let mut number = Some(number);
                while index != text.len() {
                    if let Some(digit) = ascii_to_digit(text[index]) {
                        number = number.and_then(|n| n.checked_mul(&nth(10)));
                        number = number.and_then(|n| n.checked_add(&digit));
                        index += 1;
                    } else {
                        break;
                    }
                }
                (number.map(Integer), index)
            }
            Sign::Minus => {
                let max_safe_digits = max(1, I::max_num_digits_negative(nth(10))) - 1;
                let max_safe_index = min(text.len(), max_safe_digits + offset);
                while index != max_safe_index {
                    if let Some(digit) = ascii_to_digit::<I>(text[index]) {
                        number *= nth(10);
                        number -= digit;
                        index += 1;
                    } else {
                        break;
                    }
                }
                // We parsed the digits, which do not need checking now lets see the next one:
                let mut number = Some(number);
                while index != text.len() {
                    if let Some(digit) = ascii_to_digit(text[index]) {
                        number = number.and_then(|n| n.checked_mul(&nth(10)));
                        number = number.and_then(|n| n.checked_sub(&digit));
                        index += 1;
                    } else {
                        break;
                    }
                }
                (number.map(Integer), index)
            }
        }
    }
}

impl<I> FromRadix10Checked for Integer<I>
where
    I: Zero + One + FromRadix10 + CheckedMul + CheckedAdd + MaxNumDigits,
{
    fn from_radix_10_checked(text: &[u8]) -> (Option<Self>, usize) {
        let max_safe_digits = max(1, I::max_num_digits_negative(nth(10))) - 1;
        let (number, mut index) = I::from_radix_10(&text[..min(text.len(), max_safe_digits)]);
        let mut number = Some(number);
        // We parsed the digits, which do not need checking now lets see the next one:
        while index != text.len() {
            if let Some(digit) = ascii_to_digit(text[index]) {
                number = number.and_then(|n| n.checked_mul(&nth(10)));
                number = number.and_then(|n| n.checked_add(&digit));
                index += 1;
            } else {
                break;
            }
        }
        (number.map(Integer), index)
    }
}

/// Converts an ascii character to digit
fn ascii_to_hexdigit<I>(character: u8) -> Option<I>
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
        b'a' | b'A' => Some(nth(10)),
        b'b' | b'B' => Some(nth(11)),
        b'c' | b'C' => Some(nth(12)),
        b'd' | b'D' => Some(nth(13)),
        b'e' | b'E' => Some(nth(14)),
        b'f' | b'F' => Some(nth(15)),
        _ => None,
    }
}

impl<I> FromRadix16 for Integer<I>
where
    I: Zero + One + AddAssign + MulAssign,
{
    fn from_radix_16(text: &[u8]) -> (Self, usize) {
        let mut index = 0;
        let mut number = I::zero();
        while index != text.len() {
            if let Some(digit) = ascii_to_hexdigit(text[index]) {
                number *= nth(16);
                number += digit;
                index += 1;
            } else {
                break;
            }
        }
        (Integer(number), index)
    }
}

impl<I> FromRadix16Checked for Integer<I>
where
    I: Zero + One + FromRadix16 + CheckedMul + CheckedAdd + MaxNumDigits,
{
    fn from_radix_16_checked(text: &[u8]) -> (Option<Self>, usize) {
        let max_safe_digits = max(1, I::max_num_digits_negative(nth(10))) - 1;
        let (number, mut index) = I::from_radix_16(&text[..min(text.len(), max_safe_digits)]);
        let mut number = Some(number);
        // We parsed the digits, which do not need checking now lets see the next one:
        while index != text.len() {
            if let Some(digit) = ascii_to_hexdigit(text[index]) {
                number = number.and_then(|n| n.checked_mul(&nth(16)));
                number = number.and_then(|n| n.checked_add(&digit));
                index += 1;
            } else {
                break;
            }
        }
        (number.map(Integer), index)
    }
}

impl<I> MaxNumDigits for Integer<I>
where
    I: Bounded + Zero + DivAssign + Ord + Copy,
{
    /// Returns the maximum number of digits a nonnegative representation of `I` can have depending
    /// on `radix`.
    fn max_num_digits(radix: Integer<I>) -> usize {
        let mut max = I::max_value();
        let mut d = 0;
        while max > I::zero() {
            d += 1;
            max /= radix.0;
        }
        d
    }

    /// Returns the maximum number of digits a negative representation of `I` can have depending
    /// on `radix`.
    fn max_num_digits_negative(radix: Integer<I>) -> usize {
        let mut min = I::min_value();
        let mut d = 0;
        while min < I::zero() {
            d += 1;
            min /= radix.0;
        }
        d
    }
}
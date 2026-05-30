//! Implementation for numerical types native to Rust. Currently we use macros instead of one
//! generic implementation relying on traits. This way we can better specialize some implementations
//! in the future without relying on specializations for generic implementation to become stable.
//! A generic implementation for "any integer" can still be invoked using the `Integer` wrapper.

use crate::{
    FromDigit, FromHexDigit, FromRadix10, FromRadix10Checked, FromRadix10Signed,
    FromRadix10SignedChecked, FromRadix16, FromRadix16Checked, Sign,
};

use num_traits::FromPrimitive;

use core::cmp::min;

macro_rules! impl_traits_using_integer {
    ($t:ident) => {
        impl FromRadix10 for $t {
            #[inline]
            fn from_radix_10(text: &[u8]) -> (Self, usize) {
                let mut index = 0;
                let mut number = 0;
                while index != text.len() {
                    if let Some(digit) = $t::from_digit(text[index]) {
                        number *= 10;
                        number += digit;
                        index += 1;
                    } else {
                        break;
                    }
                }
                (number, index)
            }
        }

        impl FromRadix10Signed for $t {
            #[inline]
            fn from_radix_10_signed(text: &[u8]) -> (Self, usize) {
                let mut index;
                let mut number = 0;

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
                            if let Some(digit) = $t::from_digit(text[index]) {
                                number *= 10;
                                number += digit;
                                index += 1;
                            } else {
                                break;
                            }
                        }
                    }
                    Sign::Minus => {
                        while index != text.len() {
                            if let Some(digit) = $t::from_digit(text[index]) {
                                number *= 10;
                                number -= digit;
                                index += 1;
                            } else {
                                break;
                            }
                        }
                    }
                }

                (number, index)
            }
        }

        impl FromRadix10Checked for $t {
            #[inline]
            fn from_radix_10_checked(text: &[u8]) -> (Option<Self>, usize) {
                let (number, mut index) = $t::from_radix_10(
                    &text[..min(text.len(), $t::NUM_SAFE_DIGITS_NON_NEGATIVE_RADIX_10)],
                );
                let mut number = Some(number);
                // We parsed the digits, which do not need checking now lets see the next one:
                while index != text.len() {
                    if let Some(digit) = $t::from_digit(text[index]) {
                        number = number.and_then(|n| n.checked_mul(10));
                        number = number.and_then(|n| n.checked_add(digit));
                        index += 1;
                    } else {
                        break;
                    }
                }
                (number, index)
            }
        }

        impl FromRadix10SignedChecked for $t {
            #[inline]
            fn from_radix_10_signed_checked(text: &[u8]) -> (Option<Self>, usize) {
                let mut index;
                let mut number = 0;

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
                        let max_safe_index = min(
                            text.len(),
                            $t::NUM_SAFE_DIGITS_NON_NEGATIVE_RADIX_10 + offset,
                        );
                        while index != max_safe_index {
                            if let Some(digit) = $t::from_digit(text[index]) {
                                number *= 10;
                                number += digit;
                                index += 1;
                            } else {
                                break;
                            }
                        }
                        // We parsed the digits, which do not need checking now lets see the next one:
                        let mut number = Some(number);
                        while index != text.len() {
                            if let Some(digit) = $t::from_digit(text[index]) {
                                number = number.and_then(|n| n.checked_mul(10));
                                number = number.and_then(|n| n.checked_add(digit));
                                index += 1;
                            } else {
                                break;
                            }
                        }
                        (number, index)
                    }
                    Sign::Minus => {
                        let max_safe_index = min(
                            text.len(),
                            $t::NUM_SAFE_DIGITS_NON_POSITIVE_RADIX_10 + offset,
                        );
                        while index != max_safe_index {
                            if let Some(digit) = $t::from_digit(text[index]) {
                                number *= 10;
                                number -= digit;
                                index += 1;
                            } else {
                                break;
                            }
                        }
                        // We parsed the digits, which do not need checking now lets see the next one:
                        let mut number = Some(number);
                        while index != text.len() {
                            if let Some(digit) = $t::from_digit(text[index]) {
                                number = number.and_then(|n| n.checked_mul(10));
                                number = number.and_then(|n| n.checked_sub(digit));
                                index += 1;
                            } else {
                                break;
                            }
                        }
                        (number, index)
                    }
                }
            }
        }

        impl FromRadix16 for $t {
            #[inline]
            fn from_radix_16(text: &[u8]) -> (Self, usize) {
                let mut index = 0;
                let mut number = 0;
                while index != text.len() {
                    if let Some(digit) = $t::from_hex_digit(text[index]) {
                        number *= 16;
                        number += digit;
                        index += 1;
                    } else {
                        break;
                    }
                }
                (number, index)
            }
        }

        impl FromRadix16Checked for $t {
            #[inline]
            fn from_radix_16_checked(text: &[u8]) -> (Option<Self>, usize) {
                let (number, mut index) = $t::from_radix_16(
                    &text[..min(text.len(), $t::NUM_SAFE_DIGITS_NON_NEGATIVE_RADIX_16)],
                );
                let mut number = Some(number);
                // We parsed the digits, which do not need checking now lets see the next one:
                while index != text.len() {
                    if let Some(digit) = $t::from_hex_digit(text[index]) {
                        number = number.and_then(|n| n.checked_mul(16));
                        number = number.and_then(|n| n.checked_add(digit));
                        index += 1;
                    } else {
                        break;
                    }
                }
                (number, index)
            }
        }

        impl FromDigit for $t {
            #[inline]
            fn from_digit(digit: u8) -> Option<Self> {
                match digit {
                    b'0' => Some(0),
                    b'1' => Some(1),
                    b'2' => Some(2),
                    b'3' => Some(3),
                    b'4' => Some(4),
                    b'5' => Some(5),
                    b'6' => Some(6),
                    b'7' => Some(7),
                    b'8' => Some(8),
                    b'9' => Some(9),
                    _ => None,
                }
            }
        }

        impl FromHexDigit for $t {
            #[inline]
            fn from_hex_digit(digit: u8) -> Option<Self> {
                // Unsetting the 6th bit converts ASCII alphabetic lowercase to uppercase.
                //
                // b'A' = 0b_0100_0001 (decimal 65), b'F' = 0b_0100_0110 (decimal 70)
                // b'a' = 0b_0110_0001 (decimal 97), b'f' = 0b_0110_0110 (decimal 102)
                // b'a' & 0b_1101_1111 converts 'a' to 'A'.
                let mask = 0b_1101_1111;

                if matches!(digit, b'0'..=b'9') {
                    $t::from_u8(digit - b'0')
                } else if matches!(digit & mask, b'A'..=b'F') {
                    // Subtract 55 from the result to map the character to its hexadecimal
                    // value: (65 to 70) - 55 => 10 to 15
                    $t::from_u8((digit & mask) - 55)
                } else {
                    None
                }
            }
        }
    };
}

impl_traits_using_integer!(i8);
impl_traits_using_integer!(u8);
impl_traits_using_integer!(i16);
impl_traits_using_integer!(u16);
impl_traits_using_integer!(i32);
impl_traits_using_integer!(u32);
impl_traits_using_integer!(i64);
impl_traits_using_integer!(u64);
impl_traits_using_integer!(i128);
impl_traits_using_integer!(u128);

// Num digits which are safe to parse without overflow
trait SafeDigits {
    const NUM_SAFE_DIGITS_NON_NEGATIVE_RADIX_10: usize;
    const NUM_SAFE_DIGITS_NON_NEGATIVE_RADIX_16: usize;
    const NUM_SAFE_DIGITS_NON_POSITIVE_RADIX_10: usize;
}

impl SafeDigits for i8 {
    // 127
    const NUM_SAFE_DIGITS_NON_NEGATIVE_RADIX_10: usize = 2;
    // 7F
    const NUM_SAFE_DIGITS_NON_NEGATIVE_RADIX_16: usize = 1;
    // -128
    const NUM_SAFE_DIGITS_NON_POSITIVE_RADIX_10: usize = 2;
}

impl SafeDigits for u8 {
    // 255
    const NUM_SAFE_DIGITS_NON_NEGATIVE_RADIX_10: usize = 2;
    // FF
    const NUM_SAFE_DIGITS_NON_NEGATIVE_RADIX_16: usize = 2;
    // 0
    const NUM_SAFE_DIGITS_NON_POSITIVE_RADIX_10: usize = 0;
}

impl SafeDigits for i16 {
    // 32767
    const NUM_SAFE_DIGITS_NON_NEGATIVE_RADIX_10: usize = 4;
    // 7FFF
    const NUM_SAFE_DIGITS_NON_NEGATIVE_RADIX_16: usize = 3;
    // -32768
    const NUM_SAFE_DIGITS_NON_POSITIVE_RADIX_10: usize = 5;
}

impl SafeDigits for u16 {
    // 65535
    const NUM_SAFE_DIGITS_NON_NEGATIVE_RADIX_10: usize = 4;
    // FFFF
    const NUM_SAFE_DIGITS_NON_NEGATIVE_RADIX_16: usize = 4;
    // 0
    const NUM_SAFE_DIGITS_NON_POSITIVE_RADIX_10: usize = 0;
}

impl SafeDigits for i32 {
    // 2147483647
    const NUM_SAFE_DIGITS_NON_NEGATIVE_RADIX_10: usize = 9;
    // 7FFFFFFF
    const NUM_SAFE_DIGITS_NON_NEGATIVE_RADIX_16: usize = 7;
    // -2147483648
    const NUM_SAFE_DIGITS_NON_POSITIVE_RADIX_10: usize = 9;
}

impl SafeDigits for u32 {
    // 4294967295
    const NUM_SAFE_DIGITS_NON_NEGATIVE_RADIX_10: usize = 9;
    // FFFFFFFF
    const NUM_SAFE_DIGITS_NON_NEGATIVE_RADIX_16: usize = 8;
    // 0
    const NUM_SAFE_DIGITS_NON_POSITIVE_RADIX_10: usize = 0;
}

impl SafeDigits for i64 {
    // 9223372036854775807
    const NUM_SAFE_DIGITS_NON_NEGATIVE_RADIX_10: usize = 18;
    // 7FFFFFFFFFFFFFFF
    const NUM_SAFE_DIGITS_NON_NEGATIVE_RADIX_16: usize = 15;
    // -9223372036854775808
    const NUM_SAFE_DIGITS_NON_POSITIVE_RADIX_10: usize = 18;
}

impl SafeDigits for u64 {
    // 18446744073709551615
    const NUM_SAFE_DIGITS_NON_NEGATIVE_RADIX_10: usize = 19;
    // FFFFFFFFFFFFFFFF
    const NUM_SAFE_DIGITS_NON_NEGATIVE_RADIX_16: usize = 16;
    // 0
    const NUM_SAFE_DIGITS_NON_POSITIVE_RADIX_10: usize = 0;
}

impl SafeDigits for i128 {
    // 170141183460469231731687303715884105727
    const NUM_SAFE_DIGITS_NON_NEGATIVE_RADIX_10: usize = 38;
    // 7FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF
    const NUM_SAFE_DIGITS_NON_NEGATIVE_RADIX_16: usize = 31;
    // -170141183460469231731687303715884105728
    const NUM_SAFE_DIGITS_NON_POSITIVE_RADIX_10: usize = 38;
}

impl SafeDigits for u128 {
    // 340282366920938463463374607431768211455
    const NUM_SAFE_DIGITS_NON_NEGATIVE_RADIX_10: usize = 38;
    // FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF
    const NUM_SAFE_DIGITS_NON_NEGATIVE_RADIX_16: usize = 32;
    // 0
    const NUM_SAFE_DIGITS_NON_POSITIVE_RADIX_10: usize = 0;
}

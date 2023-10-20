//! Implementation for numerical types native to Rust. Currently we use macros instead of one
//! generic implementation relying on traits. This way we can better specialize some implementations
//! in the future without relying on specializations for generic implementation to become stable.
//! A generic implementation for "any integer" can still be invoked using the `Integer` wrapper.

use crate::{
    ascii_to_digit, FromDigit, FromHexDigit, FromRadix10, FromRadix10Checked, FromRadix10Signed,
    FromRadix10SignedChecked, FromRadix16, FromRadix16Checked, Integer, MaxNumDigits,
};

macro_rules! impl_traits_using_integer {
    ($t:ident) => {
        impl FromRadix10 for $t {
            fn from_radix_10(text: &[u8]) -> (Self, usize) {
                let mut index = 0;
                let mut number = 0;
                while index != text.len() {
                    if let Some(digit) = ascii_to_digit::<$t>(text[index]) {
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
            fn from_radix_10_signed(text: &[u8]) -> (Self, usize) {
                let (Integer(i), p) = Integer::<Self>::from_radix_10_signed(text);
                (i, p)
            }
        }

        impl MaxNumDigits for $t {
            fn max_num_digits(radix: Self) -> usize {
                Integer::<Self>::max_num_digits(Integer(radix))
            }

            fn max_num_digits_negative(radix: Self) -> usize {
                Integer::<Self>::max_num_digits_negative(Integer(radix))
            }
        }

        impl FromRadix10Checked for $t {
            fn from_radix_10_checked(text: &[u8]) -> (Option<Self>, usize) {
                let (o, p) = Integer::<Self>::from_radix_10_checked(text);
                (o.map(|i| i.0), p)
            }
        }

        impl FromRadix10SignedChecked for $t {
            fn from_radix_10_signed_checked(text: &[u8]) -> (Option<Self>, usize) {
                let (o, p) = Integer::<Self>::from_radix_10_signed_checked(text);
                (o.map(|i| i.0), p)
            }
        }

        impl FromRadix16 for $t {
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
            fn from_radix_16_checked(text: &[u8]) -> (Option<Self>, usize) {
                let (o, p) = Integer::<Self>::from_radix_16_checked(text);
                (o.map(|i| i.0), p)
            }
        }

        impl FromDigit for $t {
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
            fn from_hex_digit(digit: u8) -> Option<Self> {
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
                    b'a' | b'A' => Some(10),
                    b'b' | b'B' => Some(11),
                    b'c' | b'C' => Some(12),
                    b'd' | b'D' => Some(13),
                    b'e' | b'E' => Some(14),
                    b'f' | b'F' => Some(15),
                    _ => None,
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

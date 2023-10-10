use atoi::{FromRadix10, FromRadix10Checked, FromRadix16, FromRadix16Checked, FromRadix10Signed};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::str;

pub fn i32_four_digit_number(c: &mut Criterion) {
    c.bench_function("i32 four digit number", |b| {
        b.iter(|| i32::from_radix_10(black_box(b"1996")))
    });
}

pub fn i32_four_digit_number_checked(c: &mut Criterion) {
    c.bench_function("i32 checked four digit number", |b| {
        b.iter(|| i32::from_radix_10_checked(black_box(b"1996")))
    });
}

pub fn u32_four_digit_number(c: &mut Criterion) {
    c.bench_function("u32 four digit number", |b| {
        b.iter(|| u32::from_radix_10(black_box(b"1996")))
    });
}

pub fn u32_four_digit_number_checked(c: &mut Criterion) {
    c.bench_function("u32 checked four digit number", |b| {
        b.iter(|| u32::from_radix_10_checked(black_box(b"1996")))
    });
}

pub fn i32_four_digit_hex_number(c: &mut Criterion) {
    c.bench_function("i32 four digit hex number", |b| {
        b.iter(|| i32::from_radix_16(black_box(b"1996")))
    });
}

pub fn i32_four_digit_hex_number_checked(c: &mut Criterion) {
    c.bench_function("i32 checked four digit hex number", |b| {
        b.iter(|| i32::from_radix_16_checked(black_box(b"1996")))
    });
}

pub fn u32_four_digit_hex_number(c: &mut Criterion) {
    c.bench_function("u32 four digit hex number", |b| {
        b.iter(|| u32::from_radix_16(black_box(b"1996")))
    });
}

pub fn u32_four_digit_hex_number_checked(c: &mut Criterion) {
    c.bench_function("u32 checked four digit hex number", |b| {
        b.iter(|| u32::from_radix_16_checked(black_box(b"1996")))
    });
}

pub fn i32_negative_four_digit_number(c: &mut Criterion) {
    c.bench_function("negative i32 four digit number", |b| {
        b.iter(|| i32::from_radix_10_signed(black_box(b"-1996")))
    });
}

pub fn i32_signed_four_digit_number(c: &mut Criterion) {
    c.bench_function("signed i32 four digit number", |b| {
        b.iter(|| i32::from_radix_10_signed(black_box(b"1996")))
    });
}

pub fn i32_positive_four_digit_number(c: &mut Criterion) {
    c.bench_function("positive i32 four digit number", |b| {
        b.iter(|| i32::from_radix_10_signed(black_box(b"+1996")))
    });
}

pub fn i128_signed_four_digit_number(c: &mut Criterion) {
    c.bench_function("signed i128 four digit number", |b| {
        b.iter(|| i128::from_radix_10_signed(black_box(b"1996")))
    });
}

pub fn u32_through_utf8(c: &mut Criterion) {
    c.bench_function("u32 via UTF-8", |b| {
        b.iter(|| {
            let s = str::from_utf8(black_box(b"1996")).unwrap();
            s.parse::<u32>().unwrap();
        })
    });
}

pub fn i128_through_utf8(c: &mut Criterion) {
    c.bench_function("i128 via UTF-8", |b| {
        b.iter(|| {
            let s = str::from_utf8(black_box(b"1996")).unwrap();
            s.parse::<i128>().unwrap();
        })
    });
}

criterion_group!(
    benches,
    i32_four_digit_number,
    i32_four_digit_number_checked,
    u32_four_digit_number,
    u32_four_digit_number_checked,
    i32_four_digit_hex_number,
    i32_four_digit_hex_number_checked,
    u32_four_digit_hex_number,
    u32_four_digit_hex_number_checked,
    i32_signed_four_digit_number,
    i32_negative_four_digit_number,
    i32_positive_four_digit_number,
    i128_signed_four_digit_number,
    u32_through_utf8,
    i128_through_utf8,
);
criterion_main!(benches);

#![feature(test)]
extern crate test;
extern crate atoi;
extern crate rand;
use rand::distributions::{IndependentSample, Range};
use atoi::FromRadix10;
use std::str;

// We cannot use literals, because the compiler is clever enough to remove all operations on them.
fn create_for_digit_number() -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let between = Range::new(0, 10000);
    between.ind_sample(&mut rng).to_string().into_bytes()
}

#[bench]
fn positive_four_digit_number(b: &mut test::Bencher) {
    let source = create_for_digit_number();
    b.iter(|| u32::from_radix_10(&source))
}

#[bench]
fn through_utf8(b: &mut test::Bencher) {
    let source = create_for_digit_number();
    b.iter(|| {
        let s = str::from_utf8(&source).unwrap();
        s.parse::<u32>().unwrap();
    })
}
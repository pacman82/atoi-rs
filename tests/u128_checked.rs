use atoi::{FromRadix10Checked, FromRadix10SignedChecked};

#[test]
fn signed_underflow() {
    let too_small = "-1";
    assert_eq!(
        u128::from_radix_10_signed_checked(too_small.to_string().as_bytes()),
        (None, too_small.len())
    );
}

#[test]
fn signed_overflow() {
    let too_large = "340282366920938463463374607431768211456";
    assert_eq!(
        u128::from_radix_10_signed_checked(too_large.to_string().as_bytes()),
        (None, too_large.len())
    );
}

#[test]
fn unsigned_overflow() {
    let too_large = "340282366920938463463374607431768211456";
    assert_eq!(
        u128::from_radix_10_checked(too_large.as_bytes()),
        (None, too_large.len())
    );
}

#[test]
fn unsigned_minus_one() {
    assert_eq!(u128::from_radix_10_checked("-1".as_bytes()), (Some(0), 0))
}

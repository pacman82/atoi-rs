use atoi::{FromRadix10Checked, FromRadix10SignedChecked};

type N = u8;

#[test]
fn signed_underflow() {
    let too_small = (N::MIN as i128) - 1;
    let too_small = too_small.to_string();
    assert_eq!(
        N::from_radix_10_signed_checked(too_small.as_bytes()),
        (None, too_small.len())
    );
}

#[test]
fn signed_overflow() {
    let too_large = (N::MAX as i128) + 1;
    let too_large = too_large.to_string();
    assert_eq!(
        N::from_radix_10_signed_checked(too_large.as_bytes()),
        (None, too_large.len())
    );
}

#[test]
fn unsigned_overflow() {
    let too_large = (N::MAX as u128) + 1;
    let too_large = too_large.to_string();
    assert_eq!(
        N::from_radix_10_checked(too_large.as_bytes()),
        (None, too_large.len())
    );
}

#[test]
fn unsigned_minus_one() {
    assert_eq!(N::from_radix_10_checked("-1".as_bytes()), (Some(0), 0))
}

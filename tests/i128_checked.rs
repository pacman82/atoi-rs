use atoi::{FromRadix10Checked, FromRadix10SignedChecked};

#[test]
fn signed_underflow() {
    let too_small = "170141183460469231731687303715884105729";
    assert_eq!(
        i128::from_radix_10_signed_checked(too_small.to_string().as_bytes()),
        (None, too_small.len())
    );
}

#[test]
fn signed_overflow() {
    let too_large = "170141183460469231731687303715884105728";
    assert_eq!(
        i128::from_radix_10_signed_checked(too_large.to_string().as_bytes()),
        (None, too_large.len())
    );
}

#[test]
fn unsigned_overflow() {
    let too_large = "170141183460469231731687303715884105728";
    assert_eq!(
        i128::from_radix_10_checked(too_large.as_bytes()),
        (None, too_large.len())
    );
}

#[test]
fn unsigned_minus_one() {
    assert_eq!(i128::from_radix_10_checked("-1".as_bytes()), (Some(0), 0))
}

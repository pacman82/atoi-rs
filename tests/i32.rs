use proptest::prelude::proptest;
use atoi::{FromRadix10, FromRadix10Signed};

type N = i32;

proptest! {
    #[test]
    fn roundtrip_without_sign(n in 0..=N::MAX) {
        let text = n.to_string();
        let (actual, len) = N::from_radix_10(text.as_bytes());

        assert_eq!(text.len(), len);
        assert_eq!(n, actual);
    }

    #[test]
    fn roundtrip_with_sign(n in N::MIN..=N::MAX) {
        let text = n.to_string();
        let (actual, len) = N::from_radix_10_signed(text.as_bytes());

        assert_eq!(text.len(), len);
        assert_eq!(n, actual);
    }
}
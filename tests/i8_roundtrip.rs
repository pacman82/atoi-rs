use atoi::{
    FromRadix10, FromRadix10Checked, FromRadix10Signed, FromRadix10SignedChecked, FromRadix16,
    FromRadix16Checked,
};
use proptest::prelude::proptest;

type N = i8;

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

    #[test]
    fn roundtrip_without_sign_checked(n in 0..=N::MAX) {
        let text = n.to_string();
        let (actual, len) = N::from_radix_10_checked(text.as_bytes());

        assert_eq!(text.len(), len);
        assert_eq!(Some(n), actual);
    }

    #[test]
    fn roundtrip_roundtrip_with_sign_checked(n in N::MIN..=N::MAX) {
        let text = n.to_string();
        let (actual, len) = N::from_radix_10_signed_checked(text.as_bytes());

        assert_eq!(text.len(), len);
        assert_eq!(Some(n), actual);
    }

    #[test]
    fn roundtrip_radix_16(n in 0..=N::MAX, uppercase: bool) {
        let text = if uppercase {
            format!("{n:X}")
        } else {
            format!("{n:x}")
        };
        let (actual, len) = N::from_radix_16(text.as_bytes());

        assert_eq!(text.len(), len);
        assert_eq!(n, actual);
    }

    #[test]
    fn roundtrip_radix_16_checked(n in 0..=N::MAX, uppercase: bool) {
        let text = if uppercase {
            format!("{n:X}")
        } else {
            format!("{n:x}")
        };
        let (actual, len) = N::from_radix_16_checked(text.as_bytes());

        assert_eq!(text.len(), len);
        assert_eq!(Some(n), actual);
    }
}

use core::str::FromStr;

use winnow::{
    ascii::{Int, Uint},
    stream::AsChar,
};

pub trait AsCharExt {
    /// Tests that `self` is in `1..=9`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use winnow::prelude::*;
    /// use combine::exts::AsCharExt;
    ///
    /// assert!('1'.is_nonzero_dec_digit());
    /// assert!('0'.is_nonzero_dec_digit() == false);
    /// ```
    fn is_nonzero_dec_digit(self) -> bool;
}

impl<T: AsChar> AsCharExt for T {
    #[inline]
    fn is_nonzero_dec_digit(self) -> bool {
        matches!(self.as_char(), '1'..='9')
    }
}

#[cfg(test)]
mod test {
    use super::AsCharExt;

    #[test]
    fn test_numbers() {
        for digit in 1..=9 {
            assert!(char::from_digit(digit, 10).is_some_and(|d| d.is_nonzero_dec_digit()));
        }

        assert!('0'.is_nonzero_dec_digit() == false);
    }

    #[test]
    fn test_all_u8() {
        for n in u8::MIN..=u8::MAX {
            assert_eq!(
                n.is_nonzero_dec_digit(),
                n.is_ascii_digit() && n != b'0',
                "byte `{n}` (char = `{}`).is_nonzero_dec_digit()",
                n as char,
            );
        }
    }
}

use winnow::stream::AsChar;

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

//! Parser combinators for [`winnow`].

#![no_std]

pub mod exts;

use winnow::prelude::*;

use winnow::error::ParserError;
use winnow::stream::{AsChar, Stream, StreamIsPartial};

use winnow::ascii::multispace0;

use winnow::combinator::delimited;

/// Ignores [preceding and terminating](`delimited`) [whitespace](`multispace0`) around `inner`.
///
/// # Example
///
/// ```rust
/// # use winnow::prelude::*;
/// use combine::ws;
/// fn parser<'i>(input: &mut &'i str) -> ModalResult<&'i str> {
///     ws("abc").parse_next(input)
/// }
///
/// assert_eq!(parser.parse_peek("abc"), Ok(("", "abc")));
/// assert_eq!(parser.parse_peek("abcefg"), Ok(("efg", "abc")));
/// assert_eq!(parser.parse_peek("   \tabc\n"), Ok(("", "abc")));
/// assert_eq!(parser.parse_peek(" \t\nabc\t\tefg"), Ok(("efg", "abc")));
/// assert_eq!(parser.parse_peek("abc\t\t\n  ef g"), Ok(("ef g", "abc")));
///
/// assert!(parser.parse_peek("\t\t\t\n\n\n\t").is_err());
/// assert!(parser.parse_peek("\t    a  b c").is_err());
/// ```
#[inline(always)]
pub fn ws<Input, Output, Error, ParseNext>(inner: ParseNext) -> impl Parser<Input, Output, Error>
where
    Input: Stream + StreamIsPartial,
    <Input as Stream>::Token: AsChar + Clone,
    Error: ParserError<Input>,
    ParseNext: Parser<Input, Output, Error>,
{
    delimited(multispace0, inner, multispace0)
}

#[inline(always)]
pub fn nonzero<Input, Error>(input: &mut Input) -> Result<<Input as Stream>::Slice, Error>
where
    Input: StreamIsPartial + Stream,
    <Input as Stream>::Token: AsChar,
    Error: ParserError<Input>,
{
    todo!()
}

#[cfg(test)]
mod tests {
    use winnow::prelude::*;

    use super::*;

    #[test]
    fn ws_works() {
        fn parser<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
            ws("2").parse_next(input)
        }

        assert_eq!(parser.parse_peek(" \t\n\r21c"), Ok(("1c", "2")));
        assert_eq!(parser.parse_peek("2\t\n 1c\t "), Ok(("1c\t ", "2")));
        assert!(parser.parse_peek("\t\r1342").is_err());
    }
}

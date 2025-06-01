use winnow::prelude::*;

use winnow::error::ParserError;
use winnow::stream::{AsChar, Stream, StreamIsPartial};

use winnow::ascii::multispace0;

use winnow::combinator::delimited;

/// Ignores [preceding and terminating](`delimited`) [whitespace](`multispace0`) around `inner`.
///
/// Supports complete and partial parsing.
///
/// # Effective Signature
///
/// Assuming you are parsing a `&str` [Stream]:
/// ```rust
/// # use winnow::prelude::*;
/// pub fn ws<'i>(input: &mut &'i str) -> ModalResult<&'i str>
/// # {
/// #   winnow::ascii::multispace0.parse_next(input)
/// # }
/// ```
pub fn ws<Input, Output, Error, ParseNext>(inner: ParseNext) -> impl Parser<Input, Output, Error>
where
    Input: Stream + StreamIsPartial,
    <Input as Stream>::Token: AsChar + Clone,
    Error: ParserError<Input>,
    ParseNext: Parser<Input, Output, Error>,
{
    delimited(multispace0, inner, multispace0)
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

//! Parser combinators for [`winnow`].

#![no_std]

pub mod exts;

use core::str::FromStr;

use winnow::prelude::*;

use winnow::error::ParserError;
use winnow::stream::{AsBStr, AsChar, Stream, StreamIsPartial};

use winnow::ascii::{Int, digit0, digit1, multispace0};

use winnow::combinator::{alt, delimited, dispatch, empty, fail, not, opt, trace};
use winnow::token::{any, one_of};

use exts::AsCharExt;

macro_rules! number_impl {
    (
        FnName = $FnName:ident,
        ParseF = $ParseF:expr,
    ) => {
        mod mymodule {
            #[inline]
            pub fn $FnName<Input, Output, Error>(input: &mut Input) -> Result<Output, Error>
            where
                Input: ::winnow::stream::StreamIsPartial + ::winnow::stream::Stream,
                <Input as ::winnow::stream::Stream>::Slice: ::winnow::stream::AsBStr,
                <Input as ::winnow::stream::Stream>::Token:
                    ::winnow::stream::AsChar + ::core::clone::Clone,
                Output: ::winnow::ascii::Uint,
                Error: ::winnow::error::ParserError<Input>,
            {
                use ::winnow::Parser as _;
                use ::winnow::ascii::{digit0, digit1};
                use ::winnow::combinator::{alt, dispatch, empty, fail, not, opt, trace};
                use ::winnow::stream::{AsBStr, AsChar, Stream};
                use ::winnow::token::{any, one_of};

                trace(stringify!(FnName), move |input: &mut Input| {
                    use $crate::nonzero;

                    let sign = opt(dispatch! {any.map(AsChar::as_char);
                        '+' => empty.value(true),
                        '-' => empty.value(false),
                        _ => fail,
                    });

                    alt((
                        (sign, nonzero, digit0).void(),
                        ((one_of('0'), not(digit1)).void()),
                    ))
                    .take()
                    .verify_map(|s: <Input as Stream>::Slice| {
                        let s = s.as_bstr();
                        debug_assert!(::core::str::from_utf8(s).is_ok(), "slice non-utf8: `{s:?}`");
                        // SAFETY: Only 7-bit ASCII characters are parsed
                        let s = unsafe { ::core::str::from_utf8_unchecked(s) };
                        Output::try_from_dec_uint(s)
                    })
                    .parse_next(input)
                })
                .parse_next(input)
            }
        }
    };
}

number_impl! {
    FnName = uint,
    ParseF = Uint::parse_uint,
}

/// Ignores [preceding and terminating](`delimited`) [whitespace](`multispace0`) around `inner`.
///
/// # Examples
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

/// Matches a single non-zero ASCII digit.
///
/// # Effective Signature
///
/// Assuming you're parsing a `&str` [Stream]:
/// ```rust
/// # use winnow::prelude::*;
/// fn nonzero<'i>(input: &mut &'i str) -> ModalResult<char>
/// # {
/// #   combine::nonzero.parse_next(input)
/// # }
/// ```
///
/// # Examples
///
/// ```rust
/// # use winnow::prelude::*;
/// # use winnow::error::ContextError;
/// # use winnow::Partial;
/// use combine::nonzero;
///
/// assert_eq!(nonzero::<_, ContextError>.parse_peek("1"), Ok(("", '1')));
/// assert_eq!(nonzero::<_, ContextError>.parse_peek("1023"), Ok(("023", '1')));
///
/// assert!(nonzero::<_, ContextError>.parse_peek("0").is_err());
///
/// // Partial
/// assert!(nonzero::<_, ContextError>.parse_peek("0123").is_err());
/// ```
#[inline(always)]
pub fn nonzero<Input, Error>(input: &mut Input) -> Result<<Input as Stream>::Token, Error>
where
    Input: StreamIsPartial + Stream,
    <Input as Stream>::Token: AsChar + Clone,
    Error: ParserError<Input>,
{
    one_of(<Input as Stream>::Token::is_nonzero_dec_digit).parse_next(input)
}

/// Combinator for parsing a non-zero decimal signed integer (e.g. [`i32`]).
///
/// This differs from the [`dec_int`] implementation in that leadings zeroes are
/// not accepted.
///
/// # Effective Signature
///
/// Assuming you are parsing a '&str' [Stream] into an `i32`:
/// ```rust
/// # use winnow::prelude::*;
/// pub fn number_int(input: &mut &str) -> ModalResult<i32>
/// # {
/// #   combine::number_int.parse_next(input)
/// # }
/// ```
///
/// # Examples
///
/// ```rust
/// # use winnow::prelude::*;
/// use combine::number_int;
///
/// fn parser(input: &mut &str) -> ModalResult<i32> {
///     number_int.parse_next(input)
/// }
///
/// assert_eq!(parser.parse_peek("1234"), Ok(("", 1234)));
/// assert_eq!(parser.parse_peek("-1234"), Ok(("", -1234)));
/// assert_eq!(parser.parse_peek("+1234"), Ok(("", 1234)));
/// assert_eq!(parser.parse_peek("12340"), Ok(("", 12340)));
/// assert_eq!(parser.parse_peek("0"), Ok(("", 0)));
///
/// assert!(parser.parse_peek("-0").is_err());
/// assert!(parser.parse_peek("+0").is_err());
///
/// assert!(parser.parse_peek("-").is_err());
/// assert!(parser.parse_peek("+").is_err());
/// assert!(parser.parse_peek("++123").is_err());
/// assert!(parser.parse_peek("-+123").is_err());
///
/// // unlike [`winnow`], we don't allow preceding zeroes
/// assert!(parser.parse_peek("0123").is_err(), "{:?}", parser.parse_peek("0123"));
/// assert!(parser.parse_peek("-000012").is_err());
/// assert!(parser.parse_peek(&u64::MAX.to_string()).is_err()); // overflow
/// ```
pub fn number_int<Input, Output, Error>(input: &mut Input) -> Result<Output, Error>
where
    Input: StreamIsPartial + Stream,
    <Input as Stream>::Slice: AsBStr,
    <Input as Stream>::Token: AsChar + Clone,
    Output: Int,
    Error: ParserError<Input>,
{
    trace(stringify!(number_int), move |input: &mut Input| {
        let sign = opt(dispatch! {any.map(AsChar::as_char);
            '+' => empty.value(true),
            '-' => empty.value(false),
            _ => fail,
        });
        alt((
            (sign, nonzero, digit0).void(),
            ((one_of('0'), not(digit1)).void()),
        ))
        .take()
        .verify_map(|s: <Input as Stream>::Slice| {
            let s = s.as_bstr();
            debug_assert!(core::str::from_utf8(s).is_ok(), "slice non-utf8: `{s:?}`");
            // SAFETY: Only 7-bit ASCII characters are parsed
            let s = unsafe { core::str::from_utf8_unchecked(s) };
            Output::try_from_dec_int(s)
        })
        .parse_next(input)
    })
    .parse_next(input)
}

/// Parses a string containing decimal digits to an `OutputNumber` using [`FromStr`].
pub fn number_parseable<Input, OutputNumber, Error>(
    input: &mut Input,
) -> Result<OutputNumber, Error>
where
    Input: StreamIsPartial + Stream,
    <Input as Stream>::Slice: AsBStr,
    <Input as Stream>::Token: AsChar + Clone,
    OutputNumber: FromStr,
    Error: ParserError<Input>,
{
    trace(stringify!(number_parseable), move |input: &mut Input| {
        let sign = opt(dispatch! {any.map(AsChar::as_char);
            '+' => empty.value(true),
            '-' => empty.value(false),
            _ => fail,
        });
        alt((
            (sign, nonzero, digit0).void(),
            ((one_of('0'), not(digit1)).void()),
        ))
        .take()
        .verify_map(|s: <Input as Stream>::Slice| {
            let s = s.as_bstr();
            debug_assert!(core::str::from_utf8(s).is_ok(), "slice non-utf8: `{s:?}`");
            // SAFETY: Only 7-bit ASCII characters are parsed
            let s = unsafe { core::str::from_utf8_unchecked(s) };
            <OutputNumber as FromStr>::from_str(s).ok()
        })
        .parse_next(input)
    })
    .parse_next(input)
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

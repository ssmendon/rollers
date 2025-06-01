use winnow::{
    ModalResult, Parser,
    ascii::{Caseless, digit1},
    combinator::{dispatch, peek, separated_pair, trace},
    stream::AsChar,
    token::{one_of, take_while},
};

use super::{Input, Num};

pub fn num<'i, 'a>(i: &mut Input<'i, 'a>) -> ModalResult<Num> {
    trace(
        "num",
        dispatch! {peek(one_of(AsChar::is_dec_digit));
            '0' => digit1.verify(|s: &str| s.len() == 1), // ascii zero
             _  => digit1.verify(|s: &str| s.len() <= 4), // 4-digit number
        },
    )
    .parse_to::<Num>()
    .parse_next(i)
}

pub fn dice<'i, 'a>(i: &mut Input<'i, 'a>) -> ModalResult<(Num, Num)> {
    trace("dice", separated_pair(num, Caseless("d"), num))
        .verify(|&(c, s)| c > 0 && s > 0)
        .parse_next(i)
}

pub fn label<'i, 'a>(i: &mut Input<'i, 'a>) -> ModalResult<&'i str> {
    trace(
        "label",
        take_while(1.., |c: char| {
            c.is_ascii() && c != '\\' && c != ']' && c != '['
        }),
    )
    .take()
    .parse_next(i)
}

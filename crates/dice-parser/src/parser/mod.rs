#![allow(dead_code)]

mod pieces;

use bumpalo::boxed::Box;
use bumpalo::collections::String as BString;
use pieces::{dice, label, num};
use winnow::{
    ModalResult, Parser, Stateful,
    ascii::multispace0,
    combinator::{alt, cut_err, delimited, dispatch, empty, fail, peek, trace},
    error::{ContextError, ErrMode},
    token::any,
};

type Input<'i, 'a> = Stateful<&'i str, &'a bumpalo::Bump>;
type Num = i16;

enum Expr<'arena> {
    // Operands
    Value(Num),
    Dice(Num, Num),

    // Parenthesized expression.
    Paren(Box<'arena, Expr<'arena>>),

    // Unary prefix
    Neg(Box<'arena, Expr<'arena>>),

    // Unary postfix
    Label(Box<'arena, Expr<'arena>>, BString<'arena>),

    // Binary left-associative, >),

    // Binary left-associative
    Add(Box<'arena, Expr<'arena>>, Box<'arena, Expr<'arena>>),
    Sub(Box<'arena, Expr<'arena>>, Box<'arena, Expr<'arena>>),
    Mul(Box<'arena, Expr<'arena>>, Box<'arena, Expr<'arena>>),
    Div(Box<'arena, Expr<'arena>>, Box<'arena, Expr<'arena>>),
}

impl Expr<'_> {
    fn fmt_delimited(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Expr::Value(num) => return write!(f, "{num}"),
            Expr::Dice(c, s) => return write!(f, "{c}d{s}"),
            Expr::Paren(a) => return a.fmt_delimited(f),
            _ => (),
        }

        macro_rules! unary {
            ($op:literal, $a:ident) => {{
                write!(f, $op)?;
                $a.fmt_delimited(f)?;
            }};
        }
        macro_rules! binary {
            ($op:literal, $a:ident, $b:ident) => {{
                write!(f, "{} ", $op)?;
                $a.fmt_delimited(f)?;
                write!(f, " ")?;
                $b.fmt_delimited(f)?;
            }};
        }
        write!(f, "(")?;
        match self {
            Expr::Neg(a) => unary!("-", a),
            Expr::Label(a, msg) => {
                write!(f, "[{msg}]")?;
                a.fmt_delimited(f)?;
            }
            Expr::Add(a, b) => binary!("+", a, b),
            Expr::Sub(a, b) => binary!("-", a, b),
            Expr::Mul(a, b) => binary!("*", a, b),
            Expr::Div(a, b) => binary!("/", a, b),
            _ => unreachable!(),
        }

        write!(f, ")")
    }
}

fn pratt_parser<'i, 'a>(i: &mut Input<'i, 'a>) -> ModalResult<Expr<'a>> {
    use pratt::precedence::{self, Assoc, Power};
    fn parser<'i, 'a>(
        start_power: Power,
    ) -> impl Parser<Input<'i, 'a>, Expr<'a>, ErrMode<ContextError>> {
        move |i: &mut Input<'i, 'a>| {
            precedence::precedence(
                start_power,
                trace(
                    "operand",
                    delimited(
                        multispace0,
                        dispatch! {peek(any);
                            '(' => |i: &mut Input<'i, 'a>| {
                                    delimited(
                                        '(',
                                        parser(0).map(|e| Expr::Paren(Box::new_in(e, i.state))),
                                        cut_err(')')
                                    ).parse_next(i)
                                },
                            _ => alt((
                                |i: &mut Input<'i, 'a>| {
                                    dice.map(|(c, s)| Expr::Dice(c, s)).parse_next(i)
                                },
                                num.map(Expr::Value),
                            ))
                        },
                        multispace0,
                    ),
                ),
                trace(
                    "prefix",
                    delimited(
                        multispace0,
                        dispatch! {any;
                            '+' => empty.value((18, (|_: &mut _, a| Ok(a)) as _)),
                            '-' => alt((
                                empty.value((18, (|i: &mut Input<'i, 'a>, a| Ok(Expr::Neg(Box::new_in(a, i.state)))) as _)),
                            )),
                            _ => fail,
                        },
                        multispace0,
                    ),
                ),
                trace("postfix",
                    delimited(multispace0,
                        dispatch! {any;
                            '[' => empty.value((19, (|i: &mut Input<'i, 'a>, a| {
                                let label = delimited(
                                    multispace0,
                                    label.map(|s| BString::from_str_in(s, i.state)),
                                    (multispace0, cut_err(']'), multispace0))
                                .parse_next(i)?;

                                Ok(Expr::Label(Box::new_in(a, i.state), label))
                            }) as _)),
                            _ => fail,
                        },
                        multispace0)
                ),
                trace("infix", dispatch!{any;
                    '*' => empty.value(
                        (Assoc::Left(16),
                        (|i: &mut Input<'i, 'a>, a, b| Ok(
                         Expr::Mul(Box::new_in(a, i.state),
                                   Box::new_in(b, i.state))
                    )) as _)),
                    '/' => empty.value(
                        (Assoc::Left(16),
                        (|i: &mut Input<'i, 'a>, a, b| Ok(
                            Expr::Div(Box::new_in(a, i.state),
                                      Box::new_in(b, i.state))
                        )) as _)),
                    '+' => empty.value(
                        (Assoc::Left(14),
                        (|i: &mut Input<'i, 'a>, a, b| Ok(
                            Expr::Add(Box::new_in(a, i.state),
                                      Box::new_in(b, i.state))
                        )) as _)),
                    '-' => empty.value(
                        (Assoc::Left(14),
                        (|i: &mut Input<'i, 'a>, a, b| Ok(
                            Expr::Sub(Box::new_in(a, i.state),
                                      Box::new_in(b, i.state))
                        )) as _)),
                    _ => fail,
                }

            ),
            )
            .parse_next(i)
        }
    }

    parser(0).parse_next(i)
}

#[cfg(test)]
mod test {
    use winnow::{Parser, Stateful};

    use crate::parser::Num;

    use super::{Expr, pratt_parser};

    #[allow(dead_code)]
    // to invoke fmt_delimited()
    struct PrefixNotation<'a>(Expr<'a>);

    impl core::fmt::Display for PrefixNotation<'_> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            self.0.fmt_delimited(f)
        }
    }

    fn parse(i: &str) -> Result<String, String> {
        let b = bumpalo::Bump::new();
        let i = Stateful {
            input: i,
            state: &b,
        };
        let s = pratt_parser
            .parse(i)
            .map(|r| format!("{}", PrefixNotation(r)));
        s.map_err(|e| format!("{:?}", e))
    }

    fn parse_ok(i: &str, expect: &str) {
        assert_eq!(parse(i).unwrap(), expect);
    }

    #[test]
    fn op() {
        parse_ok("   1 ", "1");
        parse_ok("    1d20", "1d20");
        parse_ok("100D5", "100d5");
        parse_ok("( 1 )", "1");

        assert!(parse("1 d20").is_err());
        assert!(parse("(123").is_err());
    }

    #[test]
    fn bad_dice() {
        assert!(parse("00001d01").is_err());
        assert!(parse("0d").is_err());
        assert!(parse("d0").is_err());
        assert!(parse("0d0").is_err());
    }

    #[test]
    fn unary() {
        parse_ok("- - 1", "(-(-1))");
        parse_ok("+ - 1", "(-1)");
        parse_ok("++ -- 1", "(-(-1))");

        assert!(parse("1 -- ++").is_err());
    }

    #[test]
    fn postfix() {
        parse_ok("1 [label]", "([label]1)");
    }

    #[test]
    fn same_precedence() {
        parse_ok("1 + 2 + 3", "(+ (+ 1 2) 3)");
        parse_ok("1 - 2 - 3", "(- (- 1 2) 3)");
        parse_ok("1 * 2 * 3", "(* (* 1 2) 3)");
        parse_ok("1 / 2 / 3", "(/ (/ 1 2) 3)");
        parse_ok("+-+1", "(-1)");
    }

    #[test]
    fn different_precedence() {
        parse_ok("1 + 2 * 3", "(+ 1 (* 2 3))");
        parse_ok("1 + 2 * 3 - 4 / 5", "(- (+ 1 (* 2 3)) (/ 4 5))");
        parse_ok("1 + 2 * 3 * 4 + 5", "(+ (+ 1 (* (* 2 3) 4)) 5)");
    }

    #[test]
    fn prefix_postfix_power() {
        parse_ok("-1 [100]", "(-([100]1))");
        parse_ok("+1[lbl]", "([lbl]1)");
    }

    #[test]
    fn bad_labels() {
        assert!(parse(" 26 [ unfinished label ").is_err());
        assert!(parse("25 [ bad label chars[ ]").is_err());
        assert!(parse("20 [ bad label chars \\]").is_err());
        assert!(parse("25 [ non-ascii ⭐]").is_err());
    }

    #[test]
    fn large_numbers() {
        parse_ok(&u8::MAX.to_string(), &u8::MAX.to_string());
        parse_ok(&u8::MIN.to_string(), &u8::MIN.to_string());

        // parse_ok(&Num::MAX.to_string(), &Num::MAX.to_string());
        // parse_ok(&u16::MAX.to_string(), &u16::MAX.to_string());

        assert!(parse(&Num::MAX.to_string()).is_err());
        assert!(parse(&u16::MAX.to_string()).is_err());
        assert!(parse(&u32::MAX.to_string()).is_err());
        assert!(parse(&u64::MAX.to_string()).is_err());
    }
}

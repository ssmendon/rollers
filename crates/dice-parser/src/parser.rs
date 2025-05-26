//! Utilities for parsing a dice roll as an [`Expr`].
//!
//! The main function is [`parse_expr`], which consumes [`Pairs`]
//! created by the [`DiceParser::parse`] method.

use std::fmt::Display;

pub use pest::Parser;

use crate::ast::Expr;
use pest::{Span, iterators::Pairs, pratt_parser::PrattParser};

#[derive(pest_derive::Parser)]
#[grammar = "dice.pest"]
/// The parsing entrypoint.
///
/// See the [`Rule::equation`] for the top-level expression.
pub struct DiceParser;

lazy_static::lazy_static! {
    /// A [`pest::pratt_parser::PrattParser`] which handles
    /// operator precedence during parsing.
    static ref PRATT_PARSER: PrattParser<Rule> = {
        use pest::pratt_parser::{Assoc::*, Op};
        use Rule::*;

        PrattParser::new()
            .op(Op::infix(add, Left) | Op::infix(subtract, Left))
            .op(Op::infix(multiply, Left) | Op::infix(divide, Left))
            .op(Op::prefix(unary_minus))
            .op(Op::postfix(label))
    };
}

// TODO: arena-based implementation?
// fn to_ast<'a, 's>(arena: &'a Bump, pairs: Pairs<'s, Rule>) -> &'a ArenaExprValue<'a, 's> {
//     PRATT_PARSER
//         .map_primary(|primary| match primary.as_rule() {
//             Rule::natural => {
//                 let v = arena.alloc(ArenaExprValue {
//                     data: ExprFrame::Int(primary.as_str().parse().unwrap()),
//                 });
//                 // let v = arena.alloc(ExprFrame::Int(primary.as_str().parse().unwrap()));
//                 // ArenaExprValue::<'a, 's> { data: v }
//                 v
//             }
//             Rule::expr => to_ast(arena, primary.into_inner()),
//             _ => todo!(),
//         })
//         .map_infix(|lhs, op, rhs| match op.as_rule() {
//             Rule::add => {
//                 let v = arena.alloc(ExprFrame::Add(lhs, rhs));
//                 todo!()
//             }
//             _ => todo!(),
//         })
//         .parse(pairs)
// }

/// Generates an [`Expr`] from a string.
///
/// # Panics
///
/// [`pest`] has a number of undocumented panics, so this function inherits that. It may panic!
///
/// # Examples
///
/// ```
/// use pest::Parser as _;
/// use dice_parser::ast::Expr;
/// use dice_parser::parser::parse_expr;
/// use dice_parser::parser::DiceParser;
/// use dice_parser::parser::Rule;
///
/// let mut pairs = DiceParser::parse(Rule::equation, "1d20 - 1");
/// let tree = Expr::sub(Expr::dice(1, 20), Expr::int(1));
///
/// if let Ok(mut pairs) = pairs {
///     let mut pairs = pairs.next().unwrap().into_inner();
///     assert_eq!(parse_expr(pairs), tree);
/// }
/// ```
pub fn parse_expr(pairs: Pairs<Rule>) -> Expr {
    PRATT_PARSER
        .map_primary(|primary| match primary.as_rule() {
            Rule::dice => {
                let mut iter = primary.into_inner();

                // required two fields: natural + d + natural
                let count = iter.next().unwrap().as_str().parse::<i32>().unwrap();
                let sides = iter.next().unwrap().as_str().parse::<i32>().unwrap();

                Expr::Dice(count, sides)
            }
            Rule::natural => Expr::Int(primary.as_str().parse::<i32>().unwrap()),
            Rule::expr => parse_expr(primary.into_inner()),
            rule => unreachable!("Expr::parse expected primary, found {:?}", rule),
        })
        .map_infix(|lhs, op, rhs| match op.as_rule() {
            Rule::add => Expr::Add(Box::new(lhs), Box::new(rhs)),
            Rule::subtract => Expr::Sub(Box::new(lhs), Box::new(rhs)),
            Rule::multiply => Expr::Mul(Box::new(lhs), Box::new(rhs)),
            Rule::divide => Expr::Div(Box::new(lhs), Box::new(rhs)),
            rule => {
                unreachable!("Expr::parse expected infix operation, found {:?}", rule)
            }
        })
        .map_prefix(|op, rhs| match op.as_rule() {
            Rule::unary_minus => Expr::Not(Box::new(rhs)),
            rule => unreachable!("Expr::parse expected prefix operation, found {:?}", rule),
        })
        .map_postfix(|lhs, op| match op.as_rule() {
            Rule::label => {
                let msg = op.into_inner().as_str().trim();
                if msg.is_empty() {
                    lhs
                } else {
                    Expr::Label(Box::new(lhs), msg)
                }
            }
            rule => {
                unreachable!("Expr::parse expected postfix operation, found {:?}", rule)
            }
        })
        .parse(pairs)
}

#[derive(Debug)]
pub struct TokenLocation {
    input: String,
    start: usize,
    end: usize,
}

impl Display for TokenLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", "")
    }
}

impl<'s> From<Span<'s>> for TokenLocation {
    fn from(value: Span) -> Self {
        Self {
            input: value.get_input().to_owned(),
            start: value.start(),
            end: value.end(),
        }
    }
}

// TODO: error handling during parsing of large numbers
#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("number must be < 4 digits, got: `{span:?}`")]
    IntTooLong { span: TokenLocation },
    #[error("{0}")]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error("number `{0}` is <= 0")]
    OutOfRange(i32),
}

pub fn try_parse_to_ast(pairs: Pairs<Rule>) -> Result<Expr<'_>, ParseError> {
    PRATT_PARSER
        .map_primary(|primary| match primary.as_rule() {
            Rule::dice => {
                let mut iter = primary.into_inner();
                let count_tok = iter.next().unwrap();
                let sides_tok = iter.next().unwrap();

                if count_tok.as_str().len() > 4 {
                    return Err(ParseError::IntTooLong {
                        span: count_tok.as_span().into(),
                    });
                }
                if sides_tok.as_str().len() > 4 {
                    return Err(ParseError::IntTooLong {
                        span: sides_tok.as_span().into(),
                    });
                }

                let count = count_tok
                    .as_str()
                    .parse::<i32>()
                    .map_err(|why| ParseError::ParseIntError(why))?;
                if count <= 0 {
                    return Err(ParseError::OutOfRange(count));
                }

                let sides = sides_tok
                    .as_str()
                    .parse::<i32>()
                    .map_err(|why| ParseError::ParseIntError(why))?;
                if sides <= 0 {
                    return Err(ParseError::OutOfRange(sides));
                }

                Ok(Expr::Dice(count, sides))
            }
            Rule::natural => {
                if primary.as_str().len() > 4 {
                    Err(ParseError::IntTooLong {
                        span: primary.as_span().into(),
                    })
                } else {
                    primary
                        .as_str()
                        .parse::<i32>()
                        .map_err(|e| ParseError::ParseIntError(e))
                        .map(|x| Expr::Int(x))
                }
            }
            Rule::expr => try_parse_to_ast(primary.into_inner()),
            _ => unreachable!(),
        })
        .map_infix(|lhs, op, rhs| match op.as_rule() {
            Rule::add => Ok(Expr::add(lhs?, rhs?)),
            Rule::subtract => Ok(Expr::sub(lhs?, rhs?)),
            Rule::multiply => Ok(Expr::mul(lhs?, rhs?)),
            Rule::divide => Ok(Expr::div(lhs?, rhs?)),
            _ => unreachable!(),
        })
        .map_prefix(|op, rhs| match op.as_rule() {
            Rule::unary_minus => Ok(Expr::not(rhs?)),
            _ => unreachable!(),
        })
        .map_postfix(|lhs, op| match op.as_rule() {
            Rule::label => {
                let msg = op.into_inner().as_str().trim();
                if msg.is_empty() {
                    Ok(lhs?)
                } else {
                    Ok(Expr::label(lhs?, msg))
                }
            }
            _ => unreachable!(),
        })
        .parse(pairs)
}

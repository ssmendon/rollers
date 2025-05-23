pub mod dice {
    pub mod ast {
        use std::{fmt::Display, str::FromStr};

        use thiserror::Error;

        #[derive(Debug)]
        pub enum Expr {
            Labeled {
                lhs: Box<Expr>,
                msg: String,
            },
            UnaryMinus(Box<Expr>),
            Dice {
                count: i32,
                sides: i32,
            },
            Natural(i32),
            BinOp {
                lhs: Box<Expr>,
                op: Op,
                rhs: Box<Expr>,
            },
        }

        impl Display for Expr {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    Expr::UnaryMinus(expr) => {
                        write!(f, "-({})", expr)
                    }
                    Expr::Dice { count, sides } => write!(f, "{}d{}", count, sides),
                    Expr::Natural(x) => Display::fmt(x, f),
                    Expr::BinOp { lhs, op, rhs } => {
                        write!(f, "({} {} {})", lhs, op, rhs)
                    }
                    Expr::Labeled { lhs, msg } => write!(f, "{}[{}]", lhs, msg),
                }
            }
        }

        #[derive(Debug)]
        pub enum Op {
            Add,
            Subtract,
            Multiply,
            Divide,
        }

        #[derive(Error, Debug)]
        #[error("invalid bin_op {0} (expected one of `+-*/`)")]
        pub struct ParseOpError(String);

        impl FromStr for Op {
            type Err = ParseOpError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    "+" => Ok(Op::Add),
                    "-" => Ok(Op::Subtract),
                    "*" => Ok(Op::Multiply),
                    "/" => Ok(Op::Divide),
                    other => Err(ParseOpError(other.to_owned())),
                }
            }
        }

        impl Display for Op {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let tok = match self {
                    Op::Add => "+",
                    Op::Subtract => "-",
                    Op::Multiply => "*",
                    Op::Divide => "/",
                };

                write!(f, "{}", tok)
            }
        }
    }

    pub mod parser {
        use std::num::NonZero;

        use super::ast::{Expr, Op};
        use pest::iterators::Pairs;
        use pest::pratt_parser::PrattParser;
        use pest_derive::Parser;

        lazy_static::lazy_static! {
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

        pub fn parse_expr(pairs: Pairs<Rule>) -> Expr {
            pest::set_call_limit(NonZero::new(512));
            PRATT_PARSER
                .map_primary(|primary| match primary.as_rule() {
                    Rule::dice => {
                        let mut iter = primary.into_inner();

                        // required two fields: natural + d + natural
                        let count = iter.next().unwrap().as_str().parse::<i32>().unwrap();
                        let sides = iter.next().unwrap().as_str().parse::<i32>().unwrap();

                        Expr::Dice { count, sides }
                    }
                    Rule::natural => Expr::Natural(primary.as_str().parse::<i32>().unwrap()),
                    Rule::expr => parse_expr(primary.into_inner()),
                    rule => unreachable!("Expr::parse expected primary, found {:?}", rule),
                })
                .map_infix(|lhs, op, rhs| {
                    let op = match op.as_rule() {
                        Rule::add => Op::Add,
                        Rule::subtract => Op::Subtract,
                        Rule::multiply => Op::Multiply,
                        Rule::divide => Op::Divide,
                        rule => {
                            unreachable!("Expr::parse expected infix operation, found {:?}", rule)
                        }
                    };
                    Expr::BinOp {
                        lhs: Box::new(lhs),
                        op,
                        rhs: Box::new(rhs),
                    }
                })
                .map_prefix(|op, rhs| match op.as_rule() {
                    Rule::unary_minus => Expr::UnaryMinus(Box::new(rhs)),
                    rule => unreachable!("Expr::parse expected prefix operation, found {:?}", rule),
                })
                .map_postfix(|lhs, op| match op.as_rule() {
                    Rule::label => {
                        // Forbid labelling a label by silently dropping it.
                        if matches!(lhs, Expr::Labeled { lhs: _, msg: _ }) {
                            return lhs;
                        }

                        let msg = op.into_inner().as_str().trim();
                        if msg.is_empty() {
                            lhs
                        } else {
                            Expr::Labeled {
                                lhs: Box::new(lhs),
                                msg: msg.to_owned(),
                            }
                        }
                    }
                    rule => {
                        unreachable!("Expr::parse expected postfix operation, found {:?}", rule)
                    }
                })
                .parse(pairs)
        }

        #[derive(Parser)]
        #[grammar = "dice.pest"]
        pub struct DiceParser;

        #[cfg(test)]
        mod test {
            use super::DiceParser;
            use super::Rule;

            use pest::consumes_to;
            use pest::parses_to;

            #[test]
            fn test_dice_parses() {
                parses_to! {
                    parser: DiceParser,
                    input: "1d20",
                    rule: Rule::dice,
                    tokens: [
                        dice(0, 4, [
                            natural(0, 1),
                            natural(2, 4),
                        ])
                    ]
                };
            }
        }
    }
}

pub mod dice {
    use pest::{iterators::Pairs, pratt_parser::PrattParser};
    use pest_derive::Parser;
    use rand::Rng as _;
    use std::fmt::Display;

    lazy_static::lazy_static! {
        static ref PRATT_PARSER: PrattParser<Rule> = {
            use pest::pratt_parser::{Assoc::*, Op};
            use Rule::*;


            PrattParser::new()
                .op(Op::infix(add, Left) | Op::infix(subtract, Left))
                .op(Op::infix(multiply, Left) | Op::infix(divide, Left))
        };
    }

    pub fn parse_expr_ast(expression: Pairs<Rule>) -> Result<Expr, Box<dyn std::error::Error>> {
        PRATT_PARSER
            .map_primary(|primary| match primary.as_rule() {
                Rule::num => Ok(Expr::Integer(primary.as_str().parse::<i32>()?)),
                Rule::dice => {
                    let mut iter = primary.as_str().split('d').map(|s| s.parse::<i32>());
                    let count = iter.next().unwrap()?;
                    let size = iter.next().unwrap()?;

                    Ok(Expr::Roll { count, size })
                }
                Rule::expr => parse_expr_ast(primary.into_inner()),
                rule => unreachable!("Expr::parse expected atom, found: {:?}", rule),
            })
            .map_infix(|lhs, op, rhs| {
                let op = match op.as_rule() {
                    Rule::add => Op::Add,
                    Rule::subtract => Op::Subtract,
                    Rule::multiply => Op::Multiply,
                    Rule::divide => Op::Divide,
                    rule => unreachable!("Expr::parse expected infix operation, found: {:?}", rule),
                };
                Ok(Expr::BinOp {
                    lhs: Box::new(lhs?),
                    op,
                    rhs: Box::new(rhs?),
                })
            })
            .parse(expression)
    }

    pub fn calculate(expression: Expr) -> i64 {
        match expression {
            Expr::Roll { count, size } => {
                if count == 0 || size == 0 {
                    0
                } else if size < 0 {
                    0
                } else {
                    let mut sum: i64 = 0;
                    let mut rng = rand::rng();
                    for _ in 0..count {
                        sum += rng.random_range(1..=size) as i64;
                    }
                    sum
                }
            }
            Expr::Integer(x) => x as i64,
            Expr::BinOp { lhs, op, rhs } => {
                let lhs = calculate(*lhs);
                let rhs = calculate(*rhs);
                match op {
                    Op::Add => lhs + rhs,
                    Op::Subtract => lhs - rhs,
                    Op::Multiply => lhs * rhs,
                    Op::Divide => lhs / rhs,
                }
            }
        }
    }

    /// A parser for dice rolls.
    ///
    /// See [`Rule`] to learn more about the grammar.
    #[derive(Parser)]
    #[grammar = "dice.pest"]
    pub struct DiceParser;

    #[derive(Debug)]
    pub enum Expr {
        Roll {
            count: i32,
            size: i32,
        },
        Integer(i32),
        BinOp {
            lhs: Box<Expr>,
            op: Op,
            rhs: Box<Expr>,
        },
    }

    impl Display for Expr {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Expr::Roll { count, size } => write!(f, "{}d{}", count, size),
                Expr::Integer(x) => write!(f, "{}", x),
                Expr::BinOp { lhs, op, rhs } => {
                    write!(f, "( {} ) {} ( {} )", lhs, op, rhs)
                }
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

    impl Display for Op {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Op::Add => write!(f, "+"),
                Op::Subtract => write!(f, "-"),
                Op::Multiply => write!(f, "*"),
                Op::Divide => write!(f, "/"),
            }
        }
    }
}

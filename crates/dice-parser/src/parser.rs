pub use pest::Parser;

use crate::ast::Expr;
use pest::{iterators::Pairs, pratt_parser::PrattParser};

#[derive(pest_derive::Parser)]
#[grammar = "dice.pest"]
pub struct DiceParser;

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
                // Forbid labelling a label by silently dropping it.
                if matches!(lhs, Expr::Label(..)) {
                    return lhs;
                }

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

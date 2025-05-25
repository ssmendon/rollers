pub(crate) mod precedence;
pub(crate) mod util;

pub mod display;
pub mod recurse;

use std::fmt::Display;

use recursion::CollapsibleExt as _;

#[derive(Debug, PartialEq)]
pub enum Expr<'s> {
    Int(i32),
    Dice(i32, i32),
    Not(Box<Expr<'s>>),
    Label(Box<Expr<'s>>, &'s str),
    Add(Box<Expr<'s>>, Box<Expr<'s>>),
    Sub(Box<Expr<'s>>, Box<Expr<'s>>),
    Mul(Box<Expr<'s>>, Box<Expr<'s>>),
    Div(Box<Expr<'s>>, Box<Expr<'s>>),
}

impl Expr<'_> {
    fn is_unit(&self) -> bool {
        self.collapse_frames(|frame| match frame {
            ExprFrame::Int(_) | ExprFrame::Dice(_, _) => true,
            ExprFrame::Not(expr) | ExprFrame::Label(expr, _) => expr,
            _ => false,
        })
    }
}

impl Display for Expr<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Int(x) => write!(f, "{}", x),
            Expr::Dice(c, s) => write!(f, "{}d{}", c, s),
            Expr::Not(rhs) => {
                if rhs.is_unit() {
                    write!(f, "-{}", rhs)
                } else {
                    write!(f, "-({})", rhs)
                }
            }
            Expr::Label(lhs, s) => {
                if lhs.is_unit() {
                    write!(f, "{}[{}]", lhs, s)
                } else {
                    write!(f, "({})[{}]", lhs, s)
                }
            }
            Expr::Add(lhs, rhs)
            | Expr::Sub(lhs, rhs)
            | Expr::Mul(lhs, rhs)
            | Expr::Div(lhs, rhs) => {
                let lop = precedence::Op::from_expr(lhs);
                let rop = precedence::Op::from_expr(rhs);
                let me = precedence::Op::from_expr(self).as_binop().unwrap();

                match precedence::BinOp::needs_parenthesis(me, lop, rop) {
                    (true, true) => write!(f, "({}) {} ({})", lhs, me.as_str(), rhs),
                    (true, false) => write!(f, "({}) {} {}", lhs, me.as_str(), rhs),
                    (false, true) => write!(f, "{} {} ({})", lhs, me.as_str(), rhs),
                    (false, false) => write!(f, "{} {} {}", lhs, me.as_str(), rhs),
                }
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ExprFrame<'s, A> {
    Int(i32),
    Dice(i32, i32),
    Not(A),
    Label(A, &'s str),
    Add(A, A),
    Sub(A, A),
    Mul(A, A),
    Div(A, A),
}

// pub struct ArenaExprValue<'a, 's> {
//     pub data: ExprFrame<'s, &'a ArenaExprValue<'a, 's>>,
// }

// impl<'s, 'a> From<ExprFrame<'s, &'a ArenaExprValue<'a, 's>>> for ArenaExprValue<'a, 's> {
//     fn from(value: ExprFrame<'s, &'a ArenaExprValue<'a, 's>>) -> Self {
//         ArenaExprValue { data: value }
//     }
// }

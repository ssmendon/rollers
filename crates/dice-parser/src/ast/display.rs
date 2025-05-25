//! Adapter types for displaying [`super::ExprFrame`] and [`super::Expr`].

// use std::fmt::Display;

// use super::{Expr, ExprFrame};

// impl Display for ExprFrame<'_, ()> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             ExprFrame::Int(x) => write!(f, "{}", x),
//             ExprFrame::Dice(c, s) => write!(f, "{}d{}", c, s),
//             ExprFrame::Not(_) => write!(f, "-_"),
//             ExprFrame::Label(_, s) => write!(f, "_[{}]", s),
//             ExprFrame::Add(_, _) => write!(f, "_ + _"),
//             ExprFrame::Sub(_, _) => write!(f, "_ - _"),
//             ExprFrame::Mul(_, _) => write!(f, "_ * _"),
//             ExprFrame::Div(_, _) => write!(f, "_ / _"),
//         }
//     }
// }

use super::{Expr, precedence};

impl std::fmt::Display for Expr<'_> {
    /// Prints the expression without any redundant parenthesis.
    ///
    /// The internals of this method use the [`precedence`] module for most
    /// of the tricky parts.
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
                let me = precedence::Op::from_expr(self)
                    .as_binop()
                    .expect("&self matched as a binary_op");

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

#[cfg(test)]
mod tests {
    use super::*;
    use Expr as e;

    #[test]
    fn test_normalized_parenthesis() {
        // 4 * (1 + 3) / 7 / ((8 + 9) * 2)
        let four = e::int(4);
        let one = e::int(1);
        let three = e::int(3);
        let seven = e::int(7);
        let eight = e::int(8);
        let nine = e::int(9);
        let two = e::int(2);

        let tree = e::div(
            e::div(e::mul(four, e::add(one, three)), seven),
            e::mul(e::add(eight, nine), two),
        );

        assert_eq!(tree.to_string(), "4 * (1 + 3) / 7 / ((8 + 9) * 2)");
    }
}

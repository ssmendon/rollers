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

use bumpalo::collections::string::String as BString;
use recursion::{Expandable, MappableFrame, PartiallyApplied};

use crate::parser::Expr;

pub mod dice;

pub enum ExprFrame<'arena, A> {
    Value(i16),
    Dice(i16, i16),

    Paren(A),

    Neg(A),

    Label(A, BString<'arena>),

    Add(A, A),
    Sub(A, A),
    Mul(A, A),
    Div(A, A),
}

impl<'arena> MappableFrame for ExprFrame<'arena, PartiallyApplied> {
    type Frame<X> = ExprFrame<'arena, X>;

    fn map_frame<A, B>(input: Self::Frame<A>, mut f: impl FnMut(A) -> B) -> Self::Frame<B> {
        match input {
            ExprFrame::Value(num) => ExprFrame::Value(num),
            ExprFrame::Dice(c, s) => ExprFrame::Dice(c, s),
            ExprFrame::Paren(a) => ExprFrame::Paren(f(a)),
            ExprFrame::Neg(a) => ExprFrame::Neg(f(a)),
            ExprFrame::Label(a, msg) => ExprFrame::Label(f(a), msg),
            ExprFrame::Add(a, b) => ExprFrame::Add(f(a), f(b)),
            ExprFrame::Sub(a, b) => ExprFrame::Sub(f(a), f(b)),
            ExprFrame::Mul(a, b) => ExprFrame::Mul(f(a), f(b)),
            ExprFrame::Div(a, b) => ExprFrame::Div(f(a), f(b)),
        }
    }
}

impl<'arena> Expandable for Expr<'arena> {
    type FrameToken = ExprFrame<'arena, PartiallyApplied>;

    fn from_frame(val: <Self::FrameToken as MappableFrame>::Frame<Self>) -> Self {
        match val {
            ExprFrame::Value(num) => Expr::Value(num),
            ExprFrame::Dice(c, s) => Expr::Dice(c, s),
            ExprFrame::Paren(a) => Expr::Paren(a),
            ExprFrame::Neg(a) => Expr::Neg(a),
            ExprFrame::Label(a, msg) => Expr::Label(a, msg),
            ExprFrame::Add(a, b) => Expr::Add(a, b),
            ExprFrame::Sub(a, b) => Expr::Sub(a, a),
            ExprFrame::Mul(a, b) => Expr::Mul(a, b),
            ExprFrame::Div(a, b) => Expr::Div(a, b),
        }
    }
}

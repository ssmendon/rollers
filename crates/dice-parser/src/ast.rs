use rand::Rng;
use recursion::{Collapsible, CollapsibleExt, Expandable, MappableFrame, PartiallyApplied};

pub fn eval(e: &Expr) -> i64 {
    let mut rng = rand::rng();

    e.collapse_frames(|frame| match frame {
        ExprFrame::Int(x) => x as i64,
        ExprFrame::Dice(c, s) => {
            (0..c).fold(0 as i64, |acc, _| acc + rng.random_range(1..=s) as i64)
        }
        ExprFrame::Not(rhs) => rhs,
        ExprFrame::Label(lhs, _) => lhs,
        ExprFrame::Add(lhs, rhs) => lhs + rhs,
        ExprFrame::Sub(lhs, rhs) => lhs - rhs,
        ExprFrame::Mul(lhs, rhs) => lhs * rhs,
        ExprFrame::Div(lhs, rhs) => lhs / rhs,
    })
}

#[derive(Debug)]
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

#[derive(Debug)]
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

impl<'a> Collapsible for &'a Expr<'a> {
    type FrameToken = ExprFrame<'a, PartiallyApplied>;

    fn into_frame(self) -> <Self::FrameToken as MappableFrame>::Frame<Self> {
        match self {
            Expr::Int(x) => ExprFrame::Int(*x),
            Expr::Dice(c, s) => ExprFrame::Dice(*c, *s),
            Expr::Not(rhs) => ExprFrame::Not(rhs.as_ref()),
            Expr::Label(lhs, s) => ExprFrame::Label(lhs.as_ref(), *s),
            Expr::Add(lhs, rhs) => ExprFrame::Add(lhs.as_ref(), rhs.as_ref()),
            Expr::Sub(lhs, rhs) => ExprFrame::Sub(lhs.as_ref(), rhs.as_ref()),
            Expr::Mul(lhs, rhs) => ExprFrame::Mul(lhs.as_ref(), rhs.as_ref()),
            Expr::Div(lhs, rhs) => ExprFrame::Div(lhs.as_ref(), rhs.as_ref()),
        }
    }
}

impl<'s> Expandable for Expr<'s> {
    type FrameToken = ExprFrame<'s, PartiallyApplied>;

    fn from_frame(val: <Self::FrameToken as MappableFrame>::Frame<Self>) -> Self {
        match val {
            ExprFrame::Int(x) => Expr::Int(x),
            ExprFrame::Dice(c, s) => Expr::Dice(c, s),
            ExprFrame::Not(rhs) => Expr::Not(Box::new(rhs)),
            ExprFrame::Label(lhs, s) => Expr::Label(Box::new(lhs), s),
            ExprFrame::Add(lhs, rhs) => Expr::Add(Box::new(lhs), Box::new(rhs)),
            ExprFrame::Sub(lhs, rhs) => Expr::Sub(Box::new(lhs), Box::new(rhs)),
            ExprFrame::Mul(lhs, rhs) => Expr::Mul(Box::new(lhs), Box::new(rhs)),
            ExprFrame::Div(lhs, rhs) => Expr::Div(Box::new(lhs), Box::new(rhs)),
        }
    }
}

impl<'s> MappableFrame for ExprFrame<'s, PartiallyApplied> {
    type Frame<X> = ExprFrame<'s, X>;

    fn map_frame<A, B>(input: Self::Frame<A>, mut f: impl FnMut(A) -> B) -> Self::Frame<B> {
        match input {
            ExprFrame::Int(x) => ExprFrame::Int(x),
            ExprFrame::Dice(c, s) => ExprFrame::Dice(c, s),
            ExprFrame::Not(rhs) => ExprFrame::Not(f(rhs)),
            ExprFrame::Label(lhs, msg) => ExprFrame::Label(f(lhs), msg),
            ExprFrame::Add(lhs, rhs) => ExprFrame::Add(f(lhs), f(rhs)),
            ExprFrame::Sub(lhs, rhs) => ExprFrame::Sub(f(lhs), f(rhs)),
            ExprFrame::Mul(lhs, rhs) => ExprFrame::Mul(f(lhs), f(rhs)),
            ExprFrame::Div(lhs, rhs) => ExprFrame::Div(f(lhs), f(rhs)),
        }
    }
}

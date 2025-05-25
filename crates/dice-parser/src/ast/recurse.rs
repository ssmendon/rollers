//! Implementations of [`recursion`] traits, and a generic [`ExprFrame::map`] method.

use recursion::{Collapsible, Expandable, MappableFrame, PartiallyApplied};

use super::{Expr, ExprFrame};

impl<'s, T> ExprFrame<'s, T> {
    /// See the documentation for [`MappableFrame::map_frame`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use recursion::Collapsible;
    /// use dice_parser::ast::{Expr, ExprFrame};
    ///
    /// // Returns the max roll possible
    /// fn roll_max(c: i32, s: i32) -> i64 { (0..c).fold(0, |acc, _| acc + s as i64) }
    ///
    /// // Result of mapping our single frame.
    /// #[derive(Debug, PartialEq)]
    /// struct RollEval<T> {
    ///     inner: T,
    ///     roll: Option<i64>,
    /// }
    ///
    /// // 5d20 * (-10 + 5)
    /// // `.map()` will evaluate exactly *one* layer:
    /// // Mul(
    /// //  f(Dice(5,20)),
    /// //  f(Add(..))
    /// // )
    /// // So, NOTHING happens to the sub-expressions of `Add`!
    /// let boxed = Expr::mul(
    ///     Expr::dice(5, 20),
    ///     Expr::add(Expr::not(Expr::int(10)), Expr::dice(3, 20)),
    /// );
    /// let frame: ExprFrame<'_, RollEval<&Expr<'_>>> = boxed.into_frame().map(|frame| match frame {
    ///     Expr::Dice(c, s) => RollEval {
    ///         inner: frame,
    ///         roll: Some(roll_max(*c, *s)),
    ///     },
    ///     inner => RollEval { inner, roll: None },
    /// });
    /// let s: String = format!("{:?}", frame);
    /// assert_eq!(
    ///     s,
    ///     "Mul(RollEval { inner: Dice(5, 20), roll: Some(100) }, RollEval { inner: Add(Not(Int(10)), Dice(3, 20)), roll: None })"
    ///    );
    /// ```
    #[inline(always)]
    pub fn map<U>(self, mut f: impl FnMut(T) -> U) -> ExprFrame<'s, U> {
        match self {
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

// Trait impls for `recursion` crate.

impl<'s> MappableFrame for ExprFrame<'s, PartiallyApplied> {
    type Frame<X> = ExprFrame<'s, X>;

    fn map_frame<A, B>(input: Self::Frame<A>, f: impl FnMut(A) -> B) -> Self::Frame<B> {
        input.map(f)
    }
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

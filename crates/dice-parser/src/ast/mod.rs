//! The [`Expr`] and [`ExprFrame`] types for working with a parsed dice roll.

pub(crate) mod precedence;
pub(crate) mod util;

pub mod display;
pub mod recurse;

use recursion::CollapsibleExt as _;

/// The [`Expr`] is the main type. It's a recursive [`Box`] enum over
/// all possible expressions in the grammar.
///
/// The `'s` lifetime is tied to the lifetime of the parsed
/// string.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
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
    /// Returns `true` if there are no binary operations from `self` until the leaf.
    fn is_unit(&self) -> bool {
        self.collapse_frames(|frame| match frame {
            ExprFrame::Int(_) | ExprFrame::Dice(_, _) => true,
            ExprFrame::Not(expr) | ExprFrame::Label(expr, _) => expr,
            _ => false,
        })
    }
}

/// A single level in the [`Expr`] tree.
///
/// See [`recursion::MappableFrame`] for more details.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
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

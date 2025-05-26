//! Helpers for comparing the precedence of operations.
//!
//! This contains two main types and various helper methods for
//! converting between them: [`BinOp`] and [`Op`].
//!
//! The most important method in this file is [`BinOp::needs_parenthesis`], which
//! is used in [`Expr`]'s [`std::fmt::Display`] implementation to remove redundant
//! parenthesis.

use std::fmt::Display;

use super::Expr;

/// A precedence value.
type Prec = i32;

/// The operations defined in the grammar.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Op {
    /// Corresponds to a no-op (e.g. a dice roll or integer).
    Empty,
    Not,
    Label,
    Add,
    Sub,
    Mul,
    Div,
}

impl Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Op::Empty => "noop",
            Op::Not => "negate",
            Op::Label => "[label]",
            Op::Add => "+",
            Op::Sub => "-",
            Op::Mul => "*",
            Op::Div => "/",
        };

        write!(f, "{}", s)
    }
}

/// Only binary operations in the grammar.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
}

impl Op {
    /// Turns an [`Expr`] into an [`Op`].
    pub const fn from_expr(expr: &Expr<'_>) -> Self {
        match expr {
            Expr::Int(_) | Expr::Dice(_, _) => Self::Empty,
            Expr::Not(..) => Self::Not,
            Expr::Label(..) => Self::Label,
            Expr::Add(..) => Self::Add,
            Expr::Sub(..) => Self::Sub,
            Expr::Mul(..) => Self::Mul,
            Expr::Div(..) => Self::Div,
        }
    }

    /// Turns an [`Op`] into a [`Prec`].
    ///
    /// Higher numbers have a higher binding precedence.
    pub const fn prec(&self) -> Prec {
        match self {
            Op::Empty => 0,
            Op::Label => 10,
            Op::Not => 20,
            Op::Add => 40,
            Op::Sub => 40,
            Op::Mul => 30,
            Op::Div => 30,
        }
    }

    /// Turns an [`Op`] into a [`BinOp`], or [`Option::None`] if it's not a binary operation.
    pub const fn as_binop(&self) -> Option<BinOp> {
        match self {
            Op::Empty | Op::Not | Op::Label => None,
            Op::Add => Some(BinOp::Add),
            Op::Sub => Some(BinOp::Sub),
            Op::Mul => Some(BinOp::Mul),
            Op::Div => Some(BinOp::Div),
        }
    }
}

impl BinOp {
    /// The string representation of the operator.
    pub const fn as_str(&self) -> &'static str {
        match self {
            BinOp::Add => "+",
            BinOp::Sub => "-",
            BinOp::Mul => "*",
            BinOp::Div => "/",
        }
    }

    /// Turns a [`BinOp`] into an [`Op`].
    pub const fn as_op(&self) -> Op {
        match self {
            BinOp::Add => Op::Add,
            BinOp::Sub => Op::Sub,
            BinOp::Mul => Op::Mul,
            BinOp::Div => Op::Div,
        }
    }

    /// See [`Op::prec`].
    pub const fn prec(&self) -> Prec {
        self.as_op().prec()
    }

    /// Determines, for a given parent `me`, left child `lop`, and right child `rop` whether parenthesis are needed.
    ///
    /// For an example of its use, see the [`std::fmt::Display`] implementation for [`Expr`].
    ///
    /// # Algorithm
    /// - If a branch's precedence is equal to `me`, and `me` is `BinOp::sub` or `BinOp::div`, we need parenthesis.
    /// - If a branch's precedence is lower than `me`, it needs parenthesis.
    ///
    /// Taken from: @worldterminator in <https://stackoverflow.com/a/58679340>
    pub const fn needs_parenthesis(me: BinOp, lop: Op, rop: Op) -> (bool, bool) {
        let lop = lop.prec();
        let rop = rop.prec()
            + match me {
                BinOp::Sub | BinOp::Div => 1, // add '1', so it's not the same precedence
                BinOp::Add | BinOp::Mul => 0,
            };
        let me = me.prec();

        (me < lop, me < rop)
    }
}

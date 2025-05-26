//! `util` provides helpful primitives for constructing [`Box`]-based [`Expr`] trees.

use super::Expr;

impl Expr<'_> {
    /// Creates an [`Expr::Int`] from `x`.
    pub const fn int(x: i32) -> Self {
        Expr::Int(x)
    }

    /// Creates an [`Expr::Dice`] from `count` and `sides`.
    pub const fn dice(count: i32, sides: i32) -> Self {
        Expr::Dice(count, sides)
    }

    /// Creates a [`Expr::Not`] with a [`Box`].
    pub fn not(rhs: Self) -> Self {
        Self::Not(Box::new(rhs))
    }

    /// Creates a [`Expr::Add`] with a [`Box`].
    pub fn add(lhs: Self, rhs: Self) -> Self {
        Self::Add(Box::new(lhs), Box::new(rhs))
    }

    /// Creates a [`Expr::Sub`] with a [`Box`].
    pub fn sub(lhs: Self, rhs: Self) -> Self {
        Self::Sub(Box::new(lhs), Box::new(rhs))
    }

    /// Creates a [`Expr::Mul`] with a [`Box`].
    pub fn mul(lhs: Self, rhs: Self) -> Self {
        Self::Mul(Box::new(lhs), Box::new(rhs))
    }

    /// Creates a [`Expr::Div`] with a [`Box`].
    pub fn div(lhs: Self, rhs: Self) -> Self {
        Self::Div(Box::new(lhs), Box::new(rhs))
    }
}

impl<'s> Expr<'s> {
    /// Creates a [`Expr::Label`] with a [`Box`].
    pub fn label(lhs: Self, msg: &'s str) -> Self {
        Expr::Label(Box::new(lhs), msg)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Make three trees and check that they produce the expected result:
    /// 1. By hand with [`Box`].
    /// 2. With the helper methods from `util`.
    /// 3. With the helper methods from `util`, but a different tree.
    fn can_construct_tree() {
        let tree1 = Expr::Mul(
            Box::new(Expr::Add(
                Box::new(Expr::Int(1)),
                Box::new(Expr::Not(Box::new(Expr::Int(5)))),
            )),
            Box::new(Expr::Label(Box::new(Expr::Int(10)), "mult 10")),
        );

        let tree2 = {
            use Expr as e;
            e::mul(
                e::add(e::int(1), e::not(e::int(5))),
                e::label(e::int(10), "mult 10"),
            )
        };

        let tree3 = {
            use Expr as e;
            e::add(e::int(1), e::int(2))
        };

        assert_eq!(tree1, tree2);
        assert_ne!(tree1, tree3);
    }
}

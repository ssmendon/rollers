//! Helper functions for computing the result of an [`crate::ast::Expr`].
//!
//! There are only two methods in this module:
//! 1. The [`eval`] function, which panics on division by zero.
//! 2. The [`try_eval`] function, which returns a [`DivideByZeroError`].

use rand::{Rng, TryCryptoRng, rngs::ThreadRng};
use recursion::CollapsibleExt as _;

use crate::ast::{Expr, ExprFrame, precedence::Op};

/// A container for a [`rand::CryptoRng`], which handles
/// all requests for dice rolls and expression evaluation.
#[derive(Debug)]
pub struct DiceRoller<R: TryCryptoRng = ThreadRng> {
    rng: R,
}

impl<R: TryCryptoRng + Rng> DiceRoller<R> {
    /// # Examples
    ///
    /// ```
    /// use dice_parser::eval::DiceRoller;
    /// let mut roller = DiceRoller::new(rand::rng());
    /// // or simply use DiceRoller::default()
    ///
    /// println!("{}", roller.roll(1, 20));
    /// ```
    #[must_use]
    pub fn new(rng: R) -> Self {
        Self { rng }
    }

    /// Rolls an `s` sided die `c` times.
    ///
    /// # Examples
    ///
    /// ```
    /// use dice_parser::eval::DiceRoller;
    ///
    /// let mut dice_roller = DiceRoller::default();
    /// let result = dice_roller.roll(1, 1); // 1d1
    /// assert_eq!(result, 1);
    ///
    /// let result = dice_roller.roll(2, 20); // 2d20
    /// assert!(result >= 2 && result <= 40);
    /// ```
    #[inline(always)]
    pub fn roll(&mut self, c: i32, s: i32) -> i64 {
        (0..c).fold(0 as i64, |acc, _| acc + self.rng.random_range(1..=s) as i64)
    }

    /// This is a non-panicking version of [`Self::eval`].
    pub fn try_eval(&mut self, e: &Expr) -> Result<i64, ArithmeticError> {
        e.try_collapse_frames(|frame| match frame {
            ExprFrame::Int(x) => Ok(x as i64),
            ExprFrame::Dice(c, s) => Ok(self.roll(c, s)),
            ExprFrame::Not(rhs) => Ok(-rhs),
            ExprFrame::Label(lhs, _) => Ok(lhs),
            ExprFrame::Add(lhs, rhs) => lhs.checked_add(rhs).map_or_else(
                || {
                    Err(ArithmeticError::Overflow {
                        lhs: Some(lhs),
                        op: Op::Add,
                        rhs: Some(rhs),
                    })
                },
                |x| Ok(x),
            ),
            ExprFrame::Sub(lhs, rhs) => lhs.checked_sub(rhs).map_or_else(
                || {
                    Err(ArithmeticError::Overflow {
                        lhs: Some(lhs),
                        op: Op::Sub,
                        rhs: Some(rhs),
                    })
                },
                |x| Ok(x),
            ),
            ExprFrame::Mul(lhs, rhs) => lhs.checked_mul(rhs).map_or_else(
                || {
                    Err(ArithmeticError::Overflow {
                        lhs: Some(lhs),
                        op: Op::Mul,
                        rhs: Some(rhs),
                    })
                },
                |x| Ok(x),
            ),
            ExprFrame::Div(lhs, rhs) => {
                if rhs != 0 {
                    Ok(lhs / rhs)
                } else {
                    Err(ArithmeticError::DivideByZero(lhs))
                }
            }
        })
    }
    /// Evaluates a parse tree and returns its result.
    ///
    /// For a version that does not panic on divide-by-zero,
    /// see [`Self::try_eval`].
    ///
    /// # Panics
    ///
    /// There is no check for division by zero. It may also
    /// panic if the program runs out of memory, but the function
    /// is stack safe as it is not recursively defined.
    /// ```
    pub fn eval(&mut self, e: &Expr) -> i64 {
        e.collapse_frames(|frame: ExprFrame<'_, i64>| match frame {
            ExprFrame::Int(x) => x as i64,
            ExprFrame::Dice(c, s) => self.roll(c, s),
            ExprFrame::Not(rhs) => -rhs,
            ExprFrame::Label(lhs, _) => lhs,
            ExprFrame::Add(lhs, rhs) => lhs + rhs,
            ExprFrame::Sub(lhs, rhs) => lhs - rhs,
            ExprFrame::Mul(lhs, rhs) => lhs * rhs,
            ExprFrame::Div(lhs, rhs) => lhs / rhs,
        })
    }
}

impl Default for DiceRoller {
    /// We use the [`rand::rng`] by default.
    fn default() -> Self {
        Self { rng: rand::rng() }
    }
}

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum ArithmeticError {
    #[error("tried to divide `{0}` by 0")]
    DivideByZero(i64),
    #[error("overflow performing `{op}` on lhs: `{lhs:?}` and rhs: `{rhs:?}`")]
    Overflow {
        lhs: Option<i64>,
        op: Op,
        rhs: Option<i64>,
    },
}

/// Represents what we attempted to divide by zero.
#[derive(thiserror::Error, Debug, PartialEq)]
#[error("tried to divide {0} by 0")]
pub struct DivideByZeroError(i64);
impl DivideByZeroError {
    pub fn new(numerator: i64) -> Self {
        Self(numerator)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use dice_mocks::*;

    #[test]
    fn test_roll_with_mock_rng() {
        let mock_crypto_rng = MockCryptoRng::default();
        let mut dr = DiceRoller::new(mock_crypto_rng);

        assert_eq!(dr.roll(1, 20), 1);
        assert_eq!(dr.roll(100, 20), 100);
    }

    #[test]
    fn test_try_eval() {
        // 1d20 + (-(4+4)/2 * (0 - 5d20)["subtraction"])
        // 21
        let tree = Expr::add(
            Expr::dice(1, 20),
            Expr::mul(
                Expr::not(Expr::div(
                    Expr::add(Expr::int(4), Expr::int(4)),
                    Expr::int(2),
                )),
                Expr::label(Expr::sub(Expr::int(0), Expr::dice(5, 20)), "subtraction"),
            ),
        );

        let mock_rng = MockCryptoRng::default();
        let mut dr = DiceRoller::new(mock_rng);

        assert_eq!(dr.try_eval(&tree), Ok(21));
    }

    #[test]
    fn test_try_eval_div_zero() {
        // 1d20 / 0
        let tree = Expr::div(Expr::dice(1, 20), Expr::int(0));
        let mut dr = DiceRoller::new(MockCryptoRng::default());

        assert_eq!(dr.try_eval(&tree), Err(ArithmeticError::DivideByZero(1)))
    }

    #[test]
    #[should_panic(expected = "attempt to divide by zero")]
    fn test_eval_div_zero_panics() {
        // 1d20 / 0
        let tree = Expr::div(Expr::dice(1, 20), Expr::int(0));
        let mut dr = DiceRoller::default();

        dr.eval(&tree); // panics
    }
}

use std::num::{NonZeroU16, TryFromIntError};

use rand::{Rng, TryCryptoRng, rngs::ThreadRng};

use crate::parser::Expr;

pub struct Roller<R: TryCryptoRng = ThreadRng> {
    rng: R,
}

impl<R: TryCryptoRng + Rng> Roller<R> {
    #[must_use]
    pub const fn new(rng: R) -> Self {
        Self { rng }
    }

    #[inline(always)]
    pub fn roll_dice(&mut self, die: Dice) -> i64 {
        (0..die.count.into()).fold(0i64, |acc, _| {
            acc + self.rng.random_range(1..=die.sides.into()) as i64
        })
    }
}

#[derive(Debug, Clone)]
pub struct Dice {
    count: NonZeroU16,
    sides: NonZeroU16,
}

impl Dice {
    pub const fn new(count: NonZeroU16, sides: NonZeroU16) -> Self {
        Dice { count, sides }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ParseDiceError {
    kind: DiceErrorKind,
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub enum DiceErrorKind {
    Invalid,
    Overflow,
    Zero,
}

impl ParseDiceError {
    pub const fn kind(&self) -> &DiceErrorKind {
        &self.kind
    }
}

impl From<TryFromIntError> for ParseDiceError {
    fn from(_: TryFromIntError) -> Self {
        ParseDiceError {
            kind: DiceErrorKind::Overflow,
        }
    }
}

impl TryFrom<Expr<'_>> for Dice {
    type Error = ParseDiceError;

    fn try_from(value: Expr<'_>) -> Result<Self, Self::Error> {
        match value {
            Expr::Dice(count, sides) => {
                let count = count.try_into()?;
                let sides = sides.try_into()?;

                if let (Some(count), Some(sides)) = (NonZeroU16::new(count), NonZeroU16::new(sides))
                {
                    Ok(Dice { count, sides })
                } else {
                    Err(ParseDiceError {
                        kind: DiceErrorKind::Zero,
                    })
                }
            }
            _ => Err(ParseDiceError {
                kind: DiceErrorKind::Invalid,
            }),
        }
    }
}

use rand::{Rng, RngCore};

use super::ArithmeticError;

pub enum Info {
    Dice {
        count: i32,
        sides: i32,

        rolls: Vec<(i32, i64)>,
        sum: i64,
    },
    Other(i64),
}

impl Info {
    pub fn roll<R: Rng>(rng: &mut R, count: i32, sides: i32) -> Result<Self, ArithmeticError> {
        if count <= 0 || sides <= 0 {
            Err(ArithmeticError::NegativeDie(count, sides))
        } else {
            let (rolls, sum) = {
                let rolls: Vec<(i32, i64)> = (0..count)
                    .map(|i| (i, rng.random_range(1..=sides) as i64))
                    .collect();

                let sum = rolls.iter().fold(0, |acc, (_, s)| acc + s);

                (rolls, sum)
            };

            Ok(Info::Dice {
                count,
                sides,
                rolls,
                sum,
            })
        }
    }
}

use dice_parser::{
    ast::{Expr, precedence::Op},
    eval::{ArithmeticError, DiceRoller, DivideByZeroError},
};
use proptest::{
    prelude::{Strategy, any},
    prop_oneof,
};
use rand::{Rng, TryCryptoRng};

pub fn naive_try_eval<R: TryCryptoRng + Rng>(
    roller: &mut DiceRoller<R>,
    expr: &Expr,
) -> Result<i64, ArithmeticError> {
    match expr {
        Expr::Int(x) => Ok(*x as i64),
        Expr::Dice(c, s) => Ok(roller.roll(*c, *s)),
        Expr::Not(expr) => naive_try_eval::<R>(roller, expr).map(|x| -x),
        Expr::Label(expr, _) => naive_try_eval::<R>(roller, expr),
        Expr::Add(lhs, rhs) | Expr::Sub(lhs, rhs) | Expr::Mul(lhs, rhs) | Expr::Div(lhs, rhs) => {
            let left = naive_try_eval::<R>(roller, lhs)?;
            let right = naive_try_eval::<R>(roller, rhs)?;

            {
                match expr {
                    Expr::Add(..) => left.checked_add(right).map_or_else(
                        || {
                            Err(ArithmeticError::Overflow {
                                lhs: Some(left),
                                op: Op::Add,
                                rhs: Some(right),
                            })
                        },
                        |x| Ok(x),
                    ),
                    Expr::Sub(..) => left.checked_sub(right).map_or_else(
                        || {
                            Err(ArithmeticError::Overflow {
                                lhs: Some(left),
                                op: Op::Sub,
                                rhs: Some(right),
                            })
                        },
                        |x| Ok(x),
                    ),
                    Expr::Mul(..) => left.checked_mul(right).map_or_else(
                        || {
                            Err(ArithmeticError::Overflow {
                                lhs: Some(left),
                                op: Op::Mul,
                                rhs: Some(right),
                            })
                        },
                        |x| Ok(x),
                    ),
                    Expr::Div(..) => {
                        if right == 0 {
                            Err(ArithmeticError::DivideByZero(left))
                        } else {
                            Ok(left / right)
                        }
                    }
                    _ => unreachable!(),
                }
            }
        }
    }
}

// Thanks to: <https://github.com/inanna-malick/recursion/blob/main/recursion-tests/src/expr/naive.rs#L70>
pub fn arb_expr<'a>() -> impl Strategy<Value = Expr<'static>> {
    let leaf = prop_oneof![
        any::<i8>().prop_map(|x| Expr::int(x as i32)),
        (any::<u8>(), any::<u8>()).prop_filter_map("zero or negative dice roll", |(a, b)| {
            if a <= 0 || b <= 0 {
                None
            } else {
                Some(Expr::dice(a as i32, b as i32))
            }
        })
    ];
    leaf.prop_recursive(8, 256, 10, move |inner| {
        prop_oneof![
            (inner.clone(), inner.clone()).prop_map(|(a, b)| Expr::add(a, b)),
            (inner.clone(), inner.clone()).prop_map(|(a, b)| Expr::sub(a, b)),
            (inner.clone(), inner.clone()).prop_map(|(a, b)| Expr::mul(a, b)),
            (inner.clone(), inner.clone()).prop_map(|(a, b)| Expr::div(a, b)),
            (inner.clone()).prop_map(|a| Expr::not(a)),
            (inner).prop_map(|a| Expr::label(a, any::<String>())),
        ]
    })
}

pub fn arb_add_expr<'a>() -> impl Strategy<Value = Expr<'static>> {
    use proptest::prelude::*;
    let leaf = prop_oneof![
        any::<i8>().prop_map(|x| Expr::int(x as i32)),
        (any::<u8>(), any::<u8>()).prop_filter_map("zero or negative dice roll", |(a, b)| {
            if a <= 0 || b <= 0 {
                None
            } else {
                Some(Expr::dice(a as i32, b as i32))
            }
        })
    ];
    leaf.prop_recursive(8, 256, 10, move |inner| {
        prop_oneof![(inner.clone(), inner).prop_map(|(a, b)| Expr::add(a, b)),]
    })
}

pub fn arb_no_div_expr<'a>() -> impl Strategy<Value = Expr<'static>> {
    use proptest::prelude::*;
    let leaf = prop_oneof![
        any::<i8>().prop_map(|x| Expr::int(x as i32)),
        (any::<u8>(), any::<u8>()).prop_filter_map("zero or negative dice roll", |(a, b)| {
            if a <= 0 || b <= 0 {
                None
            } else {
                Some(Expr::dice(a as i32, b as i32))
            }
        })
    ];
    leaf.prop_recursive(8, 256, 10, move |inner| {
        prop_oneof![
            (inner.clone()).prop_map(|a| Expr::not(a)),
            (inner.clone()).prop_map(|a| Expr::label(a, any::<String>())),
            (inner.clone(), inner.clone()).prop_map(|(a, b)| Expr::add(a, b)),
            (inner.clone(), inner.clone()).prop_map(|(a, b)| Expr::sub(a, b)),
            (inner.clone(), inner.clone()).prop_map(|(a, b)| Expr::mul(a, b)),
        ]
    })
}

mod common;
mod proptest_helpers;

use pest::Parser as _;
use proptest::prelude::*;

use dice_mocks::*;
use dice_parser::{
    ast::Expr,
    eval::DiceRoller,
    parser::{Rule, parse_expr},
};
use proptest_helpers::{arb_add_expr, arb_expr, arb_no_div_expr, naive_try_eval};

proptest! {
    #![proptest_config(ProptestConfig::with_cases(10000))]
    #[test]
    fn expr_eval(expr in arb_expr(), seed in proptest::array::uniform1(1u64..)) {
        let mut roller_gat = DiceRoller::new(MockCryptoRng::new(seed.as_ref()));
        let mut roller_copy = DiceRoller::new(MockCryptoRng::new(seed.as_ref()));
        let mut roller_naive = DiceRoller::new(MockCryptoRng::new(seed.as_ref()));

        let expr = Box::new(expr);
        let eval_gat = roller_gat.try_eval(expr.as_ref());
        if eval_gat.as_ref().is_ok() {
            assert_eq!(*eval_gat.as_ref().unwrap(), roller_copy.eval(expr.as_ref()), "eval_gat different with same random seed!");
        }
        let eval_naive = naive_try_eval::<MockCryptoRng>(&mut roller_naive, expr.as_ref());
        assert_eq!(eval_naive, eval_gat, "eval_naive and eval_gat differ!");
    }
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(10000))]
    fn expr_with_fixed_rng_equal(expr in arb_expr(), seed in proptest::array::uniform4(1u64..)) {
        let mut roller1 = DiceRoller::new(MockCryptoRng::new(seed.as_ref()));
        let mut roller2 = DiceRoller::new(MockCryptoRng::new(seed.as_ref()));
        let expr = Box::new(expr);
        let eval1 = roller1.try_eval(expr.as_ref());
        let eval2 = roller2.try_eval(expr.as_ref());
        assert_eq!(eval1, eval2, "fixed rng produced different values");
    }
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(10000))]
    #[test]
    fn expr_add_evals(expr in arb_add_expr(), seed in proptest::array::uniform1(1u64..)) {
        let mut roller_gat = DiceRoller::new(MockCryptoRng::new(seed.as_ref()));
        let mut roller_naive = DiceRoller::new(MockCryptoRng::new(seed.as_ref()));

        let expr = Box::new(expr);
        let eval_gat = roller_gat.try_eval(expr.as_ref());
        let eval_naive = naive_try_eval(&mut roller_naive, expr.as_ref());
        assert_eq!(eval_naive, eval_gat, "naive impl differs from gat impl!");

        // check also that there are no parenthesis in an expression with just addition
        assert_eq!(expr.to_string().find(|c| c == '(' || c == ')'), None, "redundant parenthesis in eval_gat!");
    }
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(10000))]
    #[test]
    fn expr_no_divs(expr in arb_no_div_expr(), seed in proptest::array::uniform1(1u64..)) {
        let mut roller = DiceRoller::new(MockCryptoRng::new(seed.as_ref()));
        let expr = Box::new(expr);
        let eval_gat = roller.try_eval(expr.as_ref());

        let mut roller_naive = DiceRoller::new(MockCryptoRng::new(seed.as_ref()));
        let eval_naive = naive_try_eval(&mut roller_naive, expr.as_ref());

        assert_eq!(eval_naive, eval_gat);
        assert!(expr.to_string().len() > 0);
    }
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(10000))]
    #[test]
    fn try_parse(text in any::<String>()) {
        if let Ok(expr) = dice_parser::parser::DiceParser::parse(Rule::equation, &text).map(|mut pairs| parse_expr(pairs.next().unwrap().into_inner())) {
            let mut roller = DiceRoller::new(MockCryptoRng::new(&[1]));
            let mut roller_naive = DiceRoller::new(MockCryptoRng::new(&[1]));
            assert_eq!(roller_naive.try_eval(&expr), roller.try_eval(&expr));
        }
    }
}

#[test]
fn proptest_regressions() {
    let cases = [
        (Expr::not(Expr::int(-1)), &[1]),
        (
            Expr::div(Expr::int(0), Expr::add(Expr::int(0), Expr::int(-1))),
            &[1],
        ),
        (
            Expr::sub(Expr::div(Expr::int(0), Expr::int(0)), Expr::int(0)),
            &[1],
        ),
        (
            Expr::sub(Expr::int(0), Expr::div(Expr::int(0), Expr::int(-1))),
            &[1],
        ),
    ];

    cases
        .into_iter()
        .map(|(input_tree, input_seed)| {
            let mut roller_gat = DiceRoller::new(MockCryptoRng::new(input_seed));
            let mut roller_naive = DiceRoller::new(MockCryptoRng::new(input_seed));
            let expr = Box::new(input_tree);

            let eval_gat = roller_gat.try_eval(expr.as_ref());
            let eval_naive = naive_try_eval(&mut roller_naive, expr.as_ref());
            assert_eq!(eval_naive, eval_gat, "naive impl differs from gat impl!");
        })
        .for_each(drop)
}

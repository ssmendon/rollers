use common::ParseEvalTest;
use dice_parser::ast::Expr;

mod common;

#[test]
fn test_parse_eval() {
    let cases = vec![ParseEvalTest {
        to_parse: "4 * (1 + 3) / 7 / ((8 + 9) * 2)",

        tree_exp: {
            use Expr as e;
            let four = e::int(4);
            let one = e::int(1);
            let three = e::int(3);
            let seven = e::int(7);
            let eight = e::int(8);
            let nine = e::int(9);
            let two = e::int(2);

            Some(e::div(
                e::div(e::mul(four, e::add(one, three)), seven),
                e::mul(e::add(eight, nine), two),
            ))
        },
        eval_exp: Some(0),

        as_str: "4 * (1 + 3) / 7 / ((8 + 9) * 2)",
        rng: None,
    }];

    for c in cases.into_iter() {
        c.doit();
    }
}

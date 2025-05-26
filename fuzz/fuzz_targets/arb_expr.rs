#![no_main]

use dice_mocks::MockCryptoRng;
use libfuzzer_sys::fuzz_target;

use dice_parser::{
    ast::{Expr, ExprFrame},
    eval::DiceRoller,
    parser::{Parser as _, Rule},
};
use recursion::*;

fuzz_target!(|data: Expr| {
    // fuzzed code goes here
    let frame = &data.into_frame();
    let depth = data.collapse_frames(|frame| match frame {
        ExprFrame::Int(_) => 1,
        ExprFrame::Dice(_, _) => 1,
        ExprFrame::Not(x) => x + 1,
        ExprFrame::Label(x, _) => x + 1,
        ExprFrame::Add(x, y) => x + y,
        ExprFrame::Sub(x, y) => x + y,
        ExprFrame::Mul(x, y) => x + y,
        ExprFrame::Div(x, y) => x + y,
    });
    assert!(depth > 0, "generated a zero-size tree");
    let parse_str = data.to_string();
    if let Ok(mut pairs) = dice_parser::parser::DiceParser::parse(Rule::equation, &parse_str) {
        let tree = dice_parser::parser::parse_expr(pairs.next().unwrap().into_inner());

        assert_eq!(
            tree, data,
            "tree parsed from data.to_string() differs from fuzz data"
        );
        assert_eq!(&tree.into_frame(), frame);

        let mut r1 = DiceRoller::new(MockCryptoRng::new(&[1, 2, 3, 4]));
        let mut r2 = DiceRoller::new(MockCryptoRng::new(&[1, 2, 3, 4]));

        assert_eq!(r1.try_eval(&data), r2.try_eval(&data));
    }
});

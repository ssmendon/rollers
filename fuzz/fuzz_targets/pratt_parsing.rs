#![no_main]

use dice_mocks::MockCryptoRng;
use dice_parser::{
    ast::Expr,
    eval::DiceRoller,
    parser::{DiceParser, Parser, Rule, parse_expr},
};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        let pairs = DiceParser::parse(Rule::equation, s);
        if let Ok(mut pairs) = pairs {
            let tree = parse_expr(pairs.next().unwrap().into_inner());

            let tree_str = tree.to_string();

            let pairs_reparse = DiceParser::parse(Rule::equation, &tree_str);
            let tree_reparse = parse_expr(pairs_reparse.unwrap().next().unwrap().into_inner());
            assert_eq!(tree, tree_reparse, "reparsed tree differs");
            assert_eq!(
                tree_str,
                tree_reparse.to_string(),
                "reparsed tree has different fmt str"
            );

            let mut dr = DiceRoller::new(MockCryptoRng::new(&[1]));
            let mut dr_reparse = DiceRoller::new(MockCryptoRng::new(&[1]));
            assert_eq!(
                dr.try_eval(&tree),
                dr_reparse.try_eval(&tree),
                "reparsed tree has different evaluation"
            );
        }
    }
});

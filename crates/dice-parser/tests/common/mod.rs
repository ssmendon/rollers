use dice_parser::ast::Expr;
use dice_parser::eval::DiceRoller;
use dice_parser::parser::{DiceParser, Rule};
use pest::Parser;

#[derive(Debug)]
pub struct ParseEvalTest<'a> {
    pub to_parse: &'static str,

    pub tree_exp: Option<Expr<'a>>,
    pub eval_exp: Option<i64>,

    pub as_str: &'static str,

    pub rng: Option<DiceRoller>,
}

impl ParseEvalTest<'_> {
    pub fn doit(mut self) {
        let pairs = DiceParser::parse(Rule::equation, self.to_parse);
        if self.tree_exp.is_none() {
            assert!(pairs.is_err());
            return;
        } else {
            assert!(pairs.is_ok());
        }

        let tree_exp = self.tree_exp.as_ref().unwrap();
        let tree_res = dice_parser::parser::parse_expr(pairs.unwrap().next().unwrap().into_inner());
        assert_eq!(&tree_res, tree_exp);

        let mut rng = std::mem::take(&mut self.rng).unwrap_or_default();
        let eval_res = rng.try_eval(&tree_res);
        if self.eval_exp.is_none() {
            assert!(eval_res.is_err());
            return;
        } else {
            assert!(eval_res.is_ok());
        }
        assert_eq!(eval_res.unwrap(), self.eval_exp.unwrap());

        assert_eq!(format!("{}", tree_res), format!("{}", self.as_str));
    }
}

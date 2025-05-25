use dice_parser::ast::Expr;
use dice_parser::eval::DiceRoller;
use dice_parser::parser::{DiceParser, Rule};
use pest::Parser;
use rand::{CryptoRng, RngCore};

use rand::rand_core;

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

/// An RNG that will always roll a '1'.
///
/// See the [`rng`] crate's book: https://rust-random.github.io/book/guide-test-fn-rng.html
#[derive(Clone, Debug)]
pub struct MockCryptoRng {
    data: Vec<u64>,
    index: usize,
}
// impls for Rng //
impl CryptoRng for MockCryptoRng {}
impl RngCore for MockCryptoRng {
    fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }
    fn next_u64(&mut self) -> u64 {
        let r = *self.data.get(self.index).unwrap_or(&0);
        self.index = (self.index + 1) % self.data.len();
        r
    }
    fn fill_bytes(&mut self, dst: &mut [u8]) {
        rand_core::impls::fill_bytes_via_next(self, dst);
    }
}
impl Default for MockCryptoRng {
    /// Always rolls a '1'!
    fn default() -> Self {
        Self {
            data: vec![5, 5, 5, 5],
            index: Default::default(),
        }
    }
}
// end impls for Rng //

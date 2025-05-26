use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use dice_parser::ast::{Expr, ExprFrame};
use dice_parser::eval::DiceRoller;
use dice_parser::parser::try_parse_to_ast;
use dice_parser::parser::{DiceParser, Rule};
use pest::Parser;
use recursion::ExpandableExt;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut test_cases = Vec::new();

    // courtesy of: https://github.com/inanna-malick/recursion/blob/main/recursion-tests/benches/expr.rs
    for depth in 17..18 {
        let big_expr = Expr::expand_frames(depth, |x| {
            if x > 0 {
                ExprFrame::Add(x - 1, x - 1)
            } else if x > 10 {
                ExprFrame::Mul(x, x + 1)
            } else {
                ExprFrame::Dice(x + 1, x + 1)
            }
        });
        test_cases.push((depth, Box::new(big_expr)));
    }

    let mut group = c.benchmark_group("print-parse-eval-loop");
    for (depth, boxed_big_expr) in test_cases.into_iter() {
        group.bench_with_input(
            BenchmarkId::new("print", depth),
            &boxed_big_expr,
            |b, expr| b.iter(|| expr.to_string()),
        );

        let to_parse = boxed_big_expr.to_string();
        group.bench_with_input(BenchmarkId::new("parse", depth), &to_parse, |b, p| {
            b.iter(|| {
                let mut pairs = DiceParser::parse(Rule::equation, &to_parse).unwrap();
                let ast = try_parse_to_ast(pairs.next().unwrap().into_inner()).unwrap();
            });
        });
        group.bench_with_input(
            BenchmarkId::new("eval", depth),
            &boxed_big_expr,
            |b, expr| {
                b.iter(|| {
                    let mut r = DiceRoller::default();
                    r.try_eval(expr.as_ref()).unwrap();
                });
            },
        );
    }
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

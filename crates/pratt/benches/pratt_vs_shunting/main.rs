use criterion::Criterion;

type Stream<'i> = &'i [u8];

static CORPUS: &str = include_str!("ariphmetic.txt");

use winnow::{
    Parser,
    ascii::digit1,
    combinator::{delimited, dispatch, empty, fail, peek},
    error::EmptyError,
    token::any,
};

fn pratt_parser(i: &mut Stream<'_>) -> winnow::Result<i64, EmptyError> {
    use pratt::precedence;
    use pratt::precedence::{Assoc, Power};

    fn parser<'i>(start_power: Power) -> impl Parser<Stream<'i>, i64, EmptyError> {
        move |i: &mut Stream<'i>| {
            {
                precedence::precedence(
                    start_power,
                    dispatch! {peek(any);
                            b'(' => delimited('(', parser(0), ')'),
                            _ => digit1.parse_to::<i64>()
                    },
                    dispatch! {any;
                        b'+' => empty.value((9, (|_: &mut _, a| Ok(a)) as _)),
                        b'-' => empty.value((9, (|_: &mut _, a: i64| Ok(-a)) as _)),
                        _ => fail,
                    },
                    fail,
                    dispatch! {any;
                        b'+' => empty.value((Assoc::Left(5), (|_: &mut _, a, b| Ok(a + b)) as _)),
                        b'-' => empty.value((Assoc::Left(5), (|_: &mut _, a, b| Ok(a - b)) as _)),
                        b'*' => empty.value((Assoc::Left(7), (|_: &mut _, a, b| Ok(a * b)) as _)),
                        b'/' => empty.value((Assoc::Left(7), (|_: &mut _, a, b| Ok(a / b)) as _)),
                        b'%' => empty.value((Assoc::Left(7), (|_: &mut _, a, b| Ok(a % b)) as _)),
                        b'^' => empty.value((Assoc::Left(9), (|_: &mut _, a, b| Ok(a ^ b)) as _)),
                        _ => fail
                    },
                )
            }
            .parse_next(i)
        }
    }

    parser(0).parse_next(i)
}
fn shunting_yard_parser(i: &mut Stream<'_>) -> winnow::Result<i64, EmptyError> {
    use pratt::precedence::{Assoc, Power};
    use pratt::shunting_yard;

    fn parser<'i>(start_power: Power) -> impl Parser<Stream<'i>, i64, EmptyError> {
        move |i: &mut Stream<'i>| {
            {
                shunting_yard::precedence(
                    start_power,
                    dispatch! {peek(any);
                            b'(' => delimited('(', parser(0), ')'),
                            _ => digit1.parse_to::<i64>()
                    },
                    dispatch! {any;
                        b'+' => empty.value((9, (|_: &mut _, a| Ok(a)) as _)),
                        b'-' => empty.value((9, (|_: &mut _, a: i64| Ok(-a)) as _)),
                        _ => fail,
                    },
                    fail,
                    dispatch! {any;
                        b'+' => empty.value((Assoc::Left(5), (|_: &mut _, a, b| Ok(a + b)) as _)),
                        b'-' => empty.value((Assoc::Left(5), (|_: &mut _, a, b| Ok(a - b)) as _)),
                        b'*' => empty.value((Assoc::Left(7), (|_: &mut _, a, b| Ok(a * b)) as _)),
                        b'/' => empty.value((Assoc::Left(7), (|_: &mut _, a, b| Ok(a / b)) as _)),
                        b'%' => empty.value((Assoc::Left(7), (|_: &mut _, a, b| Ok(a % b)) as _)),
                        b'^' => empty.value((Assoc::Left(9), (|_: &mut _, a, b| Ok(a ^ b)) as _)),
                        _ => fail
                    },
                )
            }
            .parse_next(i)
        }
    }

    parser(0).parse_next(i)
}

fn parse_expression(c: &mut Criterion) {
    // remove the last `\n`
    let input = &CORPUS.as_bytes()[0..CORPUS.as_bytes().len() - 1];
    let mut group = c.benchmark_group("pratt");

    pratt_parser.parse(input).expect("pratt should parse");
    shunting_yard_parser
        .parse(input)
        .expect("shunting yard should parse");

    group.bench_function("pratt", |b| {
        b.iter(|| std::hint::black_box(pratt_parser.parse(input).unwrap()));
    });

    group.bench_function("shunting yard", |b| {
        b.iter(|| std::hint::black_box(shunting_yard_parser.parse(input).unwrap()));
    });
}

criterion::criterion_group! {
    name = benches;
    config = Criterion::default();
    targets = parse_expression
}

criterion::criterion_main!(benches);

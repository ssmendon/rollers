use std::io::{self, BufRead as _};

use dice_parser::parser::{DiceParser, Parser as _, Rule, parse_expr, try_parse_to_ast};

fn main() {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut buffer = String::new();

    let mut dr = dice_parser::eval::DiceRoller::default();
    loop {
        buffer.clear();
        if let Err(why) = handle.read_line(&mut buffer) {
            eprintln!("Couldn't read line: {}", why);
            continue;
        }

        match DiceParser::parse(Rule::equation, &buffer) {
            Ok(mut pairs) => {
                let r = try_parse_to_ast(pairs.next().unwrap().into_inner());

                if let Ok(r) = r {
                    println!("Parsed: {:?}", r);
                    println!("Normalized: {}", r);

                    dr.try_eval(&r).map_or_else(
                        |err| eprintln!("Eval failed: {:?}", err),
                        |res| println!("Eval: {}", res),
                    );
                } else {
                    eprintln!("{}", r.unwrap_err())
                }
            }
            Err(why) => {
                eprintln!("Parse failed: {:#?}", why);
            }
        }
    }
}

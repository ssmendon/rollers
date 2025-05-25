use std::io::{self, BufRead as _};

use dice_parser::parser::{DiceParser, Parser as _, Rule, parse_expr};

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
                let r = parse_expr(pairs.next().unwrap().into_inner());
                println!("Parsed: {:?}", r);
                println!("Normalized: {}", r);

                dr.try_eval(&r).map_or_else(
                    |err| eprintln!("Eval failed: {:?}", err),
                    |res| println!("Eval: {}", res),
                );
            }
            Err(why) => {
                eprintln!("Parse failed: {:#?}", why);
            }
        }
    }
}

use std::io::{self, BufRead as _};

use pest::Parser as _;
use rollers::dice::{self, parser::Rule, parser::parse_expr};

fn main() {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut buffer = String::new();
    loop {
        buffer.clear();
        if let Err(why) = handle.read_line(&mut buffer) {
            eprintln!("Couldn't read line: {}", why);
            continue;
        }

        match dice::parser::DiceParser::parse(Rule::equation, &buffer) {
            Ok(mut pairs) => {
                let r = parse_expr(pairs.next().unwrap().into_inner());
                println!("Parsed: {:#?}", r);
            }
            Err(why) => {
                eprintln!("Parse failed: {:?}", why);
            }
        }
    }
}

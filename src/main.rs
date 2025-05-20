use std::io::{self, BufRead as _};

use pest::Parser;
use rollers::dice::{self, Rule, calculate, parse_expr_ast};

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
        match dice::DiceParser::parse(Rule::calculation, &buffer) {
            Ok(mut pairs) => {
                let r = parse_expr_ast(pairs.next().unwrap().into_inner());

                if let Ok(r) = r {
                    println!("Parsed: {:#?}", r,);
                    println!("Calculated result: {}", calculate(r))
                } else if let Err(why) = r {
                    eprintln!("Failed to parse: {:?}", why);
                }
            }
            Err(why) => {
                eprintln!("Parse failed: {:?}", why);
            }
        }
    }
}

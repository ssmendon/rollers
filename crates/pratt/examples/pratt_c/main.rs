use std::io::BufRead;

use winnow::prelude::*;

mod parser;

fn main() {
    let stdin = std::io::stdin();
    let mut handle = stdin.lock();
    let mut buffer = String::new();

    loop {
        buffer.clear();
        match handle.read_line(&mut buffer) {
            Ok(0) => break, // eof
            Ok(_) => parse(&buffer),
            Err(why) => eprintln!("failed to read line: `{why}`"),
        }
    }
}

fn parse(input: &str) {
    match parser::pratt_parser.parse(input) {
        Ok(result) => {
            println!("{result}");
        }
        Err(err) => {
            eprintln!("FAILED");
            eprintln!("{err}");
        }
    }
}

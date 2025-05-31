mod parser;

use winnow::{Stateful, prelude::*};

fn main() {
    // Join all CLI args into a single string, or just read one line from stdin
    let b = bumpalo::Bump::new();
    let args = std::env::args().skip(1).collect::<Vec<String>>().join(" ");

    let input = if args.is_empty() {
        let stdin = std::io::stdin();
        let mut buffer = String::new();
        stdin.read_line(&mut buffer).expect("Failed to read line");
        buffer
    } else {
        args
    };

    let input = Stateful {
        input: input.as_str(),
        state: &b,
    };
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

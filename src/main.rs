use std::{env, fs};
use crate::lexer::tokenize;
use crate::parser::Parser;

mod lexer;
mod parser;

fn main() {
    // Get code from file
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        println!("No file given. Use: yapko [filename]");
        return;
    }
    let code = get_file_content(&args[1]);

    let tokens = tokenize(code);
    let mut parser = Parser::new();
    parser.parse_tokens(tokens).print(0);
}

fn get_file_content(filename: &String) -> String {
    return fs::read_to_string(filename)
        .expect("Cannot open file!");
}
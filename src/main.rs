use std::{env, fs};
use std::fs::OpenOptions;
use std::io::Write;
use crate::bytecode::ByteCode;
use crate::lexer::tokenize;
use crate::parser::Parser;

mod lexer;
mod parser;
mod bytecode;

fn main() {
    // Get code from file
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        println!("No file given. Use: yapko [filename]");
        return;
    }
    let code = get_file_content(&args[1]);

    println!("code: {}", code);
    let tokens = tokenize(code);
    let parser = Parser::new();
    let node = parser.parse_tokens(tokens);

    let bytecode = ByteCode::new();

    let mut f = OpenOptions::new()
        .append(true)
        .create(true) // Optionally create the file if it doesn't already exist
        .open("a.yapkoc")
        .expect("Unable to open file");
    f.write_all(&*bytecode.generate_bytecode(node)).expect("not working idk");
}

fn get_file_content(filename: &String) -> String {
    return fs::read_to_string(filename)
        .expect("Cannot open file!");
}
use std::{env, fs};

mod lexer;

fn main() {
    // Get code from file
    let args: Vec<_> = env::args().collect();
    if !args.len() > 1 {
        println!("No file given. Use: yapko [filename]");
        return;
    }
    let code = get_file_content(&args[1]);
}

fn get_file_content(filename: &String) -> String {
    return fs::read_to_string(filename)
        .expect("Cannot open file!");
}
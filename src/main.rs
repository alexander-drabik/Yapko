use std::{env, fs};
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::{Write};
use crate::bytecode::ByteCode;
use crate::interpreter::VM;
use crate::lexer::{tokenize, TokenType};
use crate::parser::Parser;
use crate::yapko::generate_standard;

mod lexer;
mod parser;
mod bytecode;
#[path = "VM/interpreter.rs"] mod interpreter;
#[path = "VM/yapko.rs"] mod yapko;

fn main() {
    // Get code from file
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        println!("No file given. Use: yapko [filename]");
        return;
    }
    let code = get_file_content(&args[1]);
    let mut bytecode = ByteCode::new();

    let compiled_code = compile(code, &mut bytecode);

    if args.len() > 2 && args[2] == "--compile" {
        let filename = args[1][0..args[1].find(".").unwrap_or(0)].to_owned() + ".yapkoc";
        println!("Code compiled as {}", filename);
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(filename)
            .expect("Unable to open file");
        file.write_all(&*compiled_code).expect("not working idk");
    } else {
        let mut commands: HashMap<u8, String> = HashMap::new();
        for (k, v) in bytecode.commands {
            commands.insert(v, k);
        }

        //for c in &compiled_code {
        //  println!("{} {}", c, *c as char)
        //}

        let mut interpreter = VM::new();
        interpreter.scopes[0] = generate_standard();
        interpreter.interpret(compiled_code, commands);
    }
}

fn compile(code: String, bytecode: &mut ByteCode) -> Vec<u8> {
    let tokens = tokenize(code);

    //for token in &tokens {
    //    println!("{}", token.value);
    //}
    let parser = Parser::new();
    let mut compiled_code: Vec<u8> = vec![];

    let mut tokens_to_parse = vec![];
    for token in tokens {
        if matches!(token.token_type, TokenType::End) {
            if tokens_to_parse.len() > 0 {
                let node = parser.parse_tokens(tokens_to_parse.clone());
                //node.print(0);

                compiled_code.append(&mut bytecode.generate_bytecode(node));
            }
            tokens_to_parse.clear();
        } else {
            tokens_to_parse.push(token);
        }
    }

    return compiled_code;
}

fn get_file_content(filename: &String) -> String {
    return fs::read_to_string(filename)
        .expect("Cannot open file!");
}

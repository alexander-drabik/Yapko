mod lexer;

fn main() {
    let tokens = lexer::tokenize("print(1)");
    for token in tokens {
        println!("{}", token.value)
    }
}

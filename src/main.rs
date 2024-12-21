use std::fs::File;

mod lexer;

fn main() {
    let file = File::open("main.bs").unwrap();
    let lexed_stuf = lexer::start_lexing(file);
    println!("Lexed JSON: {}", lexed_stuf.to_string());
}

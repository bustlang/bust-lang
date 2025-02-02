use std::{
    fs::File,
    io::Read,
};

mod errors;
mod lexer;
mod interpreter;



fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Usage: buster <file>");
        return;
    }
    let mut file = File::open(&args[1]).unwrap();
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents).unwrap();

    let tokens = lexer::tokenize(file_contents);

    // Uncomment this to see what the tokens look like i guess
    for token in tokens.clone() {
        println!(
            "tok_type: {:?} | data: {} | body: {:?}",
            token.tok_type, token.data, token.body
        );
    }
    interpreter::interpret(tokens);
}

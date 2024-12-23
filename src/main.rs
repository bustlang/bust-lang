use std::{
    fs::File,
    io::Read,
};

mod errors;
mod lexer;
mod interpreter;

fn main() {
    let mut file = File::open("main.bs").unwrap();
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents).unwrap();

    let tokens = lexer::tokenize(file_contents);

    // Uncomment this to see what the tokens look like i guess
    /*
    for token in tokens.clone() {
        println!(
            "tok_type: {:?} | data: {} | body: {:?}",
            token.tok_type, token.data, token.body
        );
    }
    */
    interpreter::interpret(tokens);

}

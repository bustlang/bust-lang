use std::{
    fs::File,
    io::Read,
    ptr::{self, write_volatile},
    thread,
    time::Duration,
};

mod errors;
mod lexer;

fn main() {
    let mut file = File::open("main.bs").unwrap();
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents).unwrap();

    let tokens = lexer::tokenize(file_contents);
    for token in tokens {
        println!(
            "tok_type: {:?} | data: {} | body: {:?}",
            token.tok_type, token.data, token.body
        );
    }
}

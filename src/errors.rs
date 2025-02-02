use std::process::exit;

pub fn fatal(e: &str) {
    println!("Program Edged");
    println!("Fatal Error: {e}");
    exit(1);
}

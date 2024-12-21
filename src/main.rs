use std::{fs::File, thread, time::Duration};
use rand::Rng;

mod lexer;

fn main() {
    let random_npc_thread = thread::Builder::new().name("random npc thread #36741".to_string()).spawn(move || {
        thread::sleep(Duration::from_secs(rand::thread_rng().gen_range(5..9)));
        let stuf = vec!["[object Object]", "undefined", "null", "false", "Segmentation fault (core dumped)", "not a segmentation fault probably idk not sure tbh just try again i guess"];
        panic!("{}", stuf[rand::thread_rng().gen_range(0..stuf.len())]);
    }).unwrap();
    let file = File::open("main.bs").unwrap();
    let lexed_stuf = lexer::start_lexing(file);
    println!("Lexed JSON: {}", lexed_stuf.to_string());
    let _ = random_npc_thread.join();
}

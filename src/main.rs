use rand::Rng;
use std::{
    fs::File,
    ptr::{self, write_volatile},
    thread,
    time::Duration,
};

mod lexer;

fn main() {
    let random_npc_thread = thread::Builder::new()
        .name("random npc thread #36741".to_string())
        .spawn(move || {
            thread::sleep(Duration::from_secs(rand::thread_rng().gen_range(5..9)));

            // Either throw a random error (and panic) or write to address 0x00 which will cause a segfault
            let random_value_idk = rand::thread_rng().gen_range(0..10);
            match random_value_idk {
                0..5 => {
                    let stuf = vec![
                        "[object Object]",
                        "undefined",
                        "null",
                        "false",
                        "Segmentation fault (core dumped)",
                        "not a segmentation fault probably idk not sure tbh just try again i guess",
                    ];
                    panic!("{}", stuf[rand::thread_rng().gen_range(0..stuf.len())]);
                }
                _ => unsafe {
                    let mut pointer_to_protected_memory = ptr::read(0x0000 as *mut u32);
                    write_volatile(&mut pointer_to_protected_memory, 0 as u32);
                },
            }
        })
        .unwrap();
    let file = File::open("main.bs").unwrap();
    let lexed_stuf = lexer::start_lexing(file);
    println!("Lexed JSON: {}", lexed_stuf.to_string());
    let _ = random_npc_thread.join();
}

use rand::Rng;
use std::{
    fs::File, io::Read, ptr::{self, write_volatile}, thread, time::Duration
};

mod lexer;
mod errors;

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
                        "busted all over the place",
                        "error: code is not YASSSSS enough, please try again later.",
                        "error: 404 chatgpt-api not found"
                    ];
                    panic!("{}", stuf[rand::thread_rng().gen_range(0..stuf.len())]);
                }
                _ => unsafe {
                    let mut pointer_to_protected_memory = ptr::read(0x0000 as *mut u32);
                    write_volatile(&mut pointer_to_protected_memory, 0 as u32);
                },
            }
        }).unwrap();
    let mut file = File::open("main.bs").unwrap();
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents).unwrap();

    let tokens = lexer::tokenize(file_contents);
    for token in tokens {
        println!("tok_type: {:?} | data: {} | body: {:?}", token.tok_type, token.data,token.body);
    }

    let _ = random_npc_thread.join();
}

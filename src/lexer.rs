use serde_json::json;
use std::{fs::File, io::Read};

pub fn start_lexing(mut file: File) -> serde_json::Value {
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents).unwrap();

    let ret_json = serde_json::json!({"code": [
        shout_to_json((|| { let next = get_next_shout(file_contents); println!("Next shout: {next}"); return next;})())
    ]});

    return ret_json;
}

fn get_next_shout(code: String) -> String {
    let mut shout = String::new();
    let mut bracket_counter = 0;
    for c in code.chars() {
        shout = shout + c.to_string().as_str();

        if c == ';' && bracket_counter == 0 {
            break;
        } else if c == '}' {
            bracket_counter -= 1;
        } else if c == '{' {
            bracket_counter += 1;
        }
    }
    return shout;
}

fn shout_to_json(_shout: String) -> serde_json::Value {
    let mut shout = _shout.clone();
    if shout.starts_with("runnable function ") {
        shout = shout
            .strip_prefix("runnable function ")
            .unwrap()
            .to_string();
        let fun_name = shout
            .as_str()
            .chars()
            .take_while(|&ch| ch != ' ')
            .collect::<String>(); // Code stolen from StackOverflow obviously
        shout = shout.strip_prefix(fun_name.as_str()).unwrap().to_string();
        shout = shout.strip_prefix(" ").unwrap().to_string();
        let mut new_shout = String::new();
        let mut is_in_str: bool = false;
        for c in shout.chars() {
            if c.is_whitespace() && !is_in_str { continue; }
            
            new_shout = new_shout + c.to_string().as_str();
            if c == '"' {
                is_in_str = !is_in_str;
            }
        }
        return json!({
            "type": "function",
            "data": {
                "name": fun_name,
                "body": new_shout
            }
        });
    }

    return serde_json::json!({});
}

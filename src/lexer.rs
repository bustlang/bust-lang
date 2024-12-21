use serde_json::json;
use std::{fs::File, io::Read};

/// This will "convert" the raw file into a json that can then be interpreted
pub fn start_lexing(mut file: File) -> serde_json::Value {
    // First, make a string where the file contents will be stored
    let mut file_contents = String::new();
    // Then, read the file! This is ignoring errors at the moment.
    file.read_to_string(&mut file_contents).unwrap();

    let ret_json = serde_json::json!({"code": [
        shout_to_json((|| {
            let next = get_next_shout(file_contents);
            // This is really just for debugging, should be removed later
            println!("Next shout: {next}");
            return next;
        })())
    ]});

    // Finally, return the json
    return ret_json;
}

/// Will get everything until a dot, it will also respect brackets, but not strings or anything like that yet
/// A "shout" is basically just a _thing_ that will be executed and may or may not have a return value. Don't question the name
fn get_next_shout(code: String) -> String {
    let mut shout = String::new();
    // Bracket counter is used for keeping track of how many brackets were opened/closed to know when a semicolon is actually "valid"
    let mut bracket_counter = 0;
    for c in code.chars() {
        shout = shout + c.to_string().as_str();

        if c == '.' && bracket_counter == 0 {
            break;
        } else if c == '}' {
            bracket_counter -= 1;
        } else if c == '{' {
            bracket_counter += 1;
        }
    }
    return shout;
}

/// Converts a single shout (please parse it beforehand if you haven't already) into a json. Pretty WIP atm
fn shout_to_json(_shout: String) -> serde_json::Value {
    // Clone it to be able to modify
    let mut shout = _shout.clone();
    // The string literal should be replaced by some global constant or something for better code
    if shout.starts_with("runnable function ") {
        // Remove the prefix "runnable function ", the next "word" will be the function name (no checking for syntax errors yet though)
        shout = shout
            .strip_prefix("runnable function ")
            .unwrap()
            .to_string();
        // Get everything until a space is met, this is really shitty, it should also check for brackets, or generally non-alphanumeric values, but imagine doing that
        let fun_name = shout
            .as_str()
            .chars()
            .take_while(|&ch| ch != ' ')
            .collect::<String>(); // Code stolen from StackOverflow obviously
        // Remove function name from the shout, next thing should be the function body
        shout = shout.strip_prefix(fun_name.as_str()).unwrap().to_string();
        // But before we can get the function body, we first gotta remove the weird space, this is also really shitty and needs improving because it only removes ONE space
        shout = shout.strip_prefix(" ").unwrap().to_string();

        let mut new_shout = String::new();
        // Used for flipping it every time a " is encountered, so we know when to remove the spaces and when to not remove them, this also probably needs improvements, especially because there might not even exist strings in bust
        let mut is_in_str: bool = false;
        for c in shout.chars() {
            // If char is ' ' space or '\t' tab or basically any whitespace imaginable, and if not in a string, move on with the next iteration
            if c.is_whitespace() && !is_in_str { continue; }
            
            // Append the current char to the new_shout, after much (~20 seconds) of consideration, I've decided that this is the best way to do it
            new_shout = new_shout + c.to_string().as_str();
            // If we encountered a " flip the boolean
            if c == '"' {
                is_in_str = !is_in_str;
            }
        }
        // Finally, return the json with the type set to "function" and then the data. The data should have more than just "name" and "body", as body should be parsed too by the lexer of course but that will come soon™
        return json!({
            "type": "function",
            "data": {
                "name": fun_name,
                "body": new_shout
            }
        });
    }

    // If a unknown thing was encountered, just return a empty json object
    return serde_json::json!({});
}

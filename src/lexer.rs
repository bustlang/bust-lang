use crate::errors::fatal;
use serde_json::{json, Value};

#[derive(Debug, Clone)]
pub enum TokenType {
    Unknown,
    FunctionDeclaration,
    BooleanDeclaration,
    NumberDeclaration,
    DebugStatement,
    PrintStatement,
    FunctionInvokation,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub tok_type: TokenType,
    pub data: Value,
    pub body: Vec<Token>,
} /*
  impl std::fmt::Display for Token {
      fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
          writeln!(f, "Token Type: {:?}, Data: {}",&self.tok_type, &self.data)?;
          Ok(())
      }
  }*/

pub const KEYW_FUNCTION_DECL: &str = "runnable function";
pub const KEYW_BOOL_DECL: &str = "bool";
pub const KEYW_NUM_DECL: &str = "num";
pub const KEYW_DEBUG: &str = "debug";
pub const KEYW_PRINT: &str = "print";
pub const KEYW_FUNCTION_INVOKATION: &str = "run runnable function";
pub const TOK_FUNCTION_PARENTHESIES_START: char = '(';
pub const TOK_FUNCTION_PARENTHESIES_END: char = ')';
pub const TOK_START_BLOCK: char = '{';
pub const TOK_END_BLOCK: char = '}';
pub const TOK_STRING: char = '"';
pub const TOK_EOS: char = '~';
pub const TOK_VALUE: char = ':';

pub fn tokenize(_code: String) -> Vec<Token> {
    let mut code = _code.clone();
    let mut tokens = Vec::new();
    while !code.is_empty() {
        tokens.push(tok_next_expr(&mut code));
    }
    return tokens;
}

fn tok_next_expr(code: &mut String) -> Token {
    let mut tok: Token = Token {
        tok_type: TokenType::Unknown,
        data: json!({}),
        body: Vec::new(),
    };
    rem_leading_whitespace(code);
    if code.starts_with(KEYW_FUNCTION_DECL) {
        tok = handle_function_decl(code);
    } else {
        fatal("Syntax Error: Unknown keyword (1)");
    }
    return tok;
}

fn handle_function_decl(code: &mut String) -> Token {
    let mut tok: Token = Token {
        tok_type: TokenType::FunctionDeclaration,
        data: json!({}),
        body: Vec::new(),
    };
    *code = code.strip_prefix(KEYW_FUNCTION_DECL).unwrap().to_string();
    tok.tok_type = TokenType::FunctionDeclaration;
    rem_leading_whitespace(code);
    let name = code
        .chars()
        .take_while(|&c| c.is_alphanumeric())
        .collect::<String>(); // Stolen from StackOverflow
    tok.data = json!({"name": name});
    *code = code
        .strip_prefix(name.as_str())
        .unwrap_or(code)
        .to_string();
    rem_leading_whitespace(code);
    if !code.starts_with(TOK_START_BLOCK) {
        fatal(format!("Syntax Error: Expected '{TOK_START_BLOCK}'").as_str());
    }
    tok.body = tokenize_block(get_next_block(code.clone()));
    *code = code
        .strip_prefix(
            (TOK_START_BLOCK.to_string()
                + get_next_block(code.clone()).as_str()
                + TOK_END_BLOCK.to_string().as_str())
            .as_str(),
        )
        .unwrap_or(code)
        .to_string();
    return tok;
}

fn get_next_block(__code: String) -> String {
    let mut code = __code;
    let mut _code = String::new();
    let mut bracket_counter = 0;
    let mut is_in_string = false;
    if code.starts_with(TOK_START_BLOCK) {
        code = code.strip_prefix(TOK_START_BLOCK).unwrap().to_string();
        bracket_counter += 1;
    }
    for c in code.chars() {
        match c {
            TOK_START_BLOCK => {
                if is_in_string {
                    break;
                }
                bracket_counter += 1;
            }
            TOK_END_BLOCK => {
                if is_in_string {
                    break;
                }
                bracket_counter -= 1;
            }
            TOK_STRING => {
                is_in_string = !is_in_string;
            }
            _ => {}
        }
        if bracket_counter == 0 {
            break;
        }
        _code = _code + c.to_string().as_str();
    }
    return _code;
}

fn tokenize_block(_code: String) -> Vec<Token> {
    let mut code = _code.clone();
    let mut tokens: Vec<Token> = Vec::new();

    if code.starts_with(TOK_START_BLOCK) {
        code = code.strip_prefix(TOK_START_BLOCK).unwrap().to_string();
    }
    if code.ends_with(TOK_END_BLOCK) {
        code = code.strip_suffix(TOK_END_BLOCK).unwrap().to_string();
    }

    rem_leading_whitespace(&mut code);
    while !code.is_empty() {
        if code.starts_with(KEYW_DEBUG) {
            code = code.strip_prefix(KEYW_DEBUG).unwrap().to_string();
            rem_leading_whitespace(&mut code);
            let stuf = get_all_until_eos(&code);
            code = code.strip_prefix(stuf.as_str()).unwrap().to_string();
            tokens.push(Token {
                tok_type: TokenType::DebugStatement,
                data: json!({"str": stuf.strip_suffix(TOK_EOS).unwrap()}),
                body: Vec::new(),
            });
        } else if code.starts_with(KEYW_PRINT) {
            code = code.strip_prefix(KEYW_PRINT).unwrap().to_string();
            rem_leading_whitespace(&mut code);
            let stuf = get_all_until_eos(&code);
            code = code.strip_prefix(stuf.as_str()).unwrap().to_string();
            tokens.push(Token {
                tok_type: TokenType::PrintStatement,
                data: json!({"str": stuf.strip_suffix(TOK_EOS).unwrap()}),
                body: Vec::new(),
            });
        } else if code.starts_with(KEYW_FUNCTION_INVOKATION) {
            code = code
                .strip_prefix(KEYW_FUNCTION_INVOKATION)
                .unwrap()
                .to_string();
            rem_leading_whitespace(&mut code);
            let stuf = get_all_until_eos(&code);
            code = code.strip_prefix(stuf.as_str()).unwrap().to_string();
            let mut func_name = String::new();
            for c in stuf.chars() {
                if !c.is_alphanumeric() && c != TOK_FUNCTION_PARENTHESIES_START {
                    fatal("Unexpected token");
                }
                if c == TOK_FUNCTION_PARENTHESIES_START {
                    // ignore everything else (for now)
                    break;
                }
                func_name = func_name + c.to_string().as_str();
            }
            tokens.push(Token {
                tok_type: TokenType::FunctionInvokation,
                data: json!({"name": func_name}),
                body: Vec::new(),
            });
        } else if code.starts_with(KEYW_BOOL_DECL) {
            code = code.strip_prefix(KEYW_BOOL_DECL).unwrap().to_string();
            rem_leading_whitespace(&mut code);
            let stuf = get_all_until_eos(&code);
            let mut morestuf = stuf.split(':').nth(1).unwrap().to_string();
            rem_leading_whitespace(&mut morestuf);
            code = code.strip_prefix(stuf.as_str()).unwrap().to_string();
            tokens.push(Token {
                tok_type: TokenType::BooleanDeclaration,
                data: json!({"name": stuf.strip_suffix(TOK_EOS).unwrap().split(TOK_VALUE).nth(0).unwrap().to_string(), "value": morestuf.strip_suffix(TOK_EOS).unwrap().to_string()}),
                body: Vec::new(),
            });
        } else if code.starts_with(KEYW_NUM_DECL) {
            code = code.strip_prefix(KEYW_NUM_DECL).unwrap().to_string();
            rem_leading_whitespace(&mut code);
            let stuf = get_all_until_eos(&code);
            let mut morestuf = stuf.split(':').nth(1).unwrap().to_string();
            rem_leading_whitespace(&mut morestuf);
            code = code.strip_prefix(stuf.as_str()).unwrap().to_string();
            tokens.push(Token {
                tok_type: TokenType::NumberDeclaration,
                data: json!({"name": stuf.strip_suffix(TOK_EOS).unwrap().split(TOK_VALUE).nth(0).unwrap().to_string(), "value": morestuf.strip_suffix(TOK_EOS).unwrap().to_string()}),
                body: Vec::new(),
            });
        } else {
            fatal("Syntax Error: Unknown Keyword (2)");
        }
        rem_leading_whitespace(&mut code);
    }

    return tokens;
}

/// Will include the eos
fn get_all_until_eos(code: &String) -> String {
    let mut ret = String::new();

    let mut is_in_string = false;
    let mut bracket_counter = 0;
    for c in code.chars() {
        ret = ret + c.to_string().as_str();
        match c {
            TOK_STRING => {
                is_in_string = !is_in_string;
            }
            TOK_START_BLOCK => {
                if !is_in_string {
                    bracket_counter += 1;
                }
            }
            TOK_END_BLOCK => {
                if !is_in_string {
                    bracket_counter -= 1;
                }
            }
            TOK_EOS => {
                if !is_in_string && bracket_counter == 0 {
                    return ret;
                }
            }
            _ => {}
        }
    }
    return ret;
}

fn rem_leading_whitespace(_str: &mut String) {
    let mut new_str = _str.clone();
    for c in _str.chars() {
        if c.is_whitespace() {
            new_str = new_str
                .strip_prefix(c.to_string().as_str())
                .unwrap_or(new_str.as_str())
                .to_string();
        } else {
            break;
        }
    }
    *_str = new_str;
}

use crate::errors::fatal;
use crate::lexer::Token;
use crate::lexer::TokenType;

#[derive(Clone, PartialEq, Debug)]
enum VariableType {
    Number,
    Boolean,
    Function,
}

#[derive(Clone, Debug)]
enum VariableValueType {
    Str,
    Number,
    Boolean,
    TokenVec,
}

#[derive(Clone, Debug)]
struct VariableValue {
    value_type: VariableValueType,
    str_value: String,
    num_value: i128,
    bool_value: bool,
    token_vec_value: Vec<Token>,
}

#[derive(Clone, Debug)]
struct Variable {
    variable_type: VariableType,
    value: VariableValue,
}

#[derive(Clone, Debug)]
struct Context {
    variables: Vec<Variable>,
}

pub fn interpret(tokens: Vec<Token>) {
    let mut context = Context {
        variables: Vec::new(),
    };
    for token in tokens {
        interpret_token(&mut context, token);
    }
    // println!("Final context: {:?}", context);
}

fn interpret_token(context: &mut Context, token: Token) {
    match token.tok_type {
        TokenType::DebugStatement => println!("[Debug] {}", token.data["str"]),
        TokenType::FunctionDeclaration => {
            context.variables.push(Variable {
                variable_type: VariableType::Function,
                value: VariableValue {
                    value_type: VariableValueType::Str,
                    str_value: token.data["fun_name"].to_string(),
                    num_value: 0,
                    bool_value: false,
                    token_vec_value: token.body.clone(),
                },
            });

            for tok in token.body {
                interpret_token(context, tok);
            }
        }
        TokenType::FunctionInvokation => 'fi_block: {
            for variable in context.variables.clone() {
                if variable.variable_type == VariableType::Function
                    && variable.value.str_value == token.data["fun_name"].to_string()
                {
                    for tok in variable.value.token_vec_value.clone() {
                        interpret_token(context, tok);
                    }
                    break 'fi_block;
                }
            }
            // now, this error isn't actually fatal, but imagine making it easy for the programmer
            fatal(&format!(
                "Undeclared function \"{}\"",
                token.data["fun_name"].as_str().unwrap()
            ));
        }
        _ => fatal("Unknown Token Type (how did this even happen)"),
    }
}

// Stolen from StackOverflow, of course
fn rem_first_and_last(value: &str) -> &str {
    let mut chars = value.chars();
    chars.next();
    chars.next_back();
    chars.as_str()
}

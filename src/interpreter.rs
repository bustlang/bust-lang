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
    num_value: f64,
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
        TokenType::PrintStatement => 'ps_block: {
            for variable in context.variables.clone() {
                if variable.variable_type == VariableType::Boolean
                    && variable.value.str_value == token.data["str"].to_string()
                {
                    println!("{}", variable.value.bool_value);
                    break 'ps_block;
                } else if variable.variable_type == VariableType::Number
                    && variable.value.str_value == token.data["str"].to_string()
                {
                    println!("{}", variable.value.num_value);
                    break 'ps_block;
                }
            }
            println!("{}", token.data["str"])
        }
        TokenType::FunctionDeclaration => {
            for variable in context.variables.clone() {
                if variable.value.str_value == token.data["name"].to_string() {
                    fatal("Names must be unique!");
                }
            }
            context.variables.push(Variable {
                variable_type: VariableType::Function,
                value: VariableValue {
                    value_type: VariableValueType::Str,
                    str_value: token.data["name"].to_string(),
                    num_value: 0.0,
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
                    && variable.value.str_value == token.data["name"].to_string()
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
                token.data["name"].as_str().unwrap()
            ));
        }
        TokenType::BooleanDeclaration => {
            for variable in context.variables.clone() {
                if variable.value.str_value == token.data["name"].to_string() {
                    fatal("Names must be unique!");
                }
            }
            if token.data["value"].as_str().unwrap() != "true"
                && token.data["value"].as_str().unwrap() != "false"
            {
                fatal("Booleans must be either true or false");
            }
            context.variables.push(Variable {
                variable_type: VariableType::Boolean,
                value: VariableValue {
                    value_type: VariableValueType::Boolean,
                    str_value: token.data["name"].to_string(),
                    num_value: 0.0,
                    bool_value: token.data["value"].as_str().unwrap().parse().unwrap(),
                    token_vec_value: token.body.clone(),
                },
            });
        }
        TokenType::NumberDeclaration => {
            for variable in context.variables.clone() {
                if variable.value.str_value == token.data["name"].to_string() {
                    fatal("Names must be unique!");
                }
            }
            if token.data["value"]
                .as_str()
                .unwrap()
                .parse::<f64>()
                .is_err()
            {
                fatal(
                    format!("{} is not a number", token.data["value"].as_str().unwrap()).as_str(),
                );
            }
            context.variables.push(Variable {
                variable_type: VariableType::Number,
                value: VariableValue {
                    value_type: VariableValueType::Number,
                    str_value: token.data["name"].to_string(),
                    num_value: token.data["value"].as_str().unwrap().parse().unwrap(),
                    bool_value: false,
                    token_vec_value: token.body.clone(),
                },
            });
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

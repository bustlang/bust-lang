use crate::errors::fatal;
use crate::lexer::Token;
use crate::lexer::TokenType;

#[derive(Clone)]
enum VariableType {
    Number,
    Boolean
} 

#[derive(Clone)]
struct Variable {
    variable_type: VariableType
}

#[derive(Clone)]
struct Context {
    variables: Vec<Variable>
}

pub fn interpret(tokens: Vec<Token>) {
    for token in tokens {
        interpret_token(Context {variables: Vec::new()}, token);
    }
}

fn interpret_token(context: Context, token: Token) {
    match token.tok_type {
        TokenType::DebugStatement => println!("[Debug] {}",token.data["str"]),
        TokenType::FunctionDeclaration => {
            for tok in token.body {
                interpret_token(context.clone(), tok);
            }
        },
        _ => fatal("Unknown Token Type (how did this even happen)")
    }
}
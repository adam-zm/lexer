use crate::lexer::Token;
use crate::lexer::TokenType;
use crate::parser::Operation;

mod lexer;
mod parser;

fn main() -> Result<(), miette::Report> {
    let input = "var name;
        23 + 32;";

    let mut lexer = lexer::Lexer::initialize(input);
    let mut tokens: Vec<Token> = Vec::new();
    loop {
        let token = lexer.next()?;
        tokens.push(token.clone());
        if token.token_t == TokenType::EOF {
            break;
        }
    }

    let mut ast: Vec<Operation> = Vec::new();
    let mut parser = parser::Parser::initilize(&tokens, &mut ast);
    parser.parse_token()?;
    parser.parse_token()?;

    for op in ast {
        match &op {
            Operation::CreateVar(name) => {
                println!("CreateVar {}", name);
            }
            Operation::Addition(a, b) => {
                println!("Addition {} + {}", a, b);
            }
        }
    }

    Ok(())
}

use crate::lexer::Token;
use crate::lexer::TokenType;

mod lexer;
mod parser;

fn main() -> Result<(), miette::Report> {
    let input = "var *name = 32
        ";

    let mut lexer = lexer::Lexer::initialize(input);
    let mut tokens: Vec<Token> = Vec::new();
    loop {
        let token = lexer.next()?;
        tokens.push(token.clone());
        if token.token_t == TokenType::EOF {
            break;
        }
    }

    let mut parser = parser::Parser::initilize(&tokens);
    parser.parse_token()?;

    Ok(())
}

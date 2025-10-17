use crate::lexer::Token;
use crate::lexer::TokenType;

mod lexer;
mod parser;

fn main() -> Result<(), miette::Report> {
    let input = "**(){}/!= == = /\"this is a string\"**(((}23 while fun fun = nil() while_test";

    let mut lexer = lexer::Lexer::initialize(input);
    let mut tokens: Vec<Token> = Vec::new();
    loop {
        let token = lexer.next()?;
        tokens.push(token.clone());
        if token.token_t == TokenType::EOF {
            break;
        }
    }

    let parser = parser::Parser::initilize(tokens);
    parser.list();

    Ok(())
}

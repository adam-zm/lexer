use crate::lexer::TokenType;

mod lexer;

fn main() -> Result<(), miette::Report> {
    let input = "**(){}/!= == = /\"this is a string\"**(((}23 while fun fun = nil() while_test";

    let mut lexer = lexer::Lexer::initialize(input);
    loop {
        let token = lexer.next()?;
        println!("{:?}", &token);
        if token.token_t == TokenType::EOF {
            break;
        }
    }

    Ok(())
}

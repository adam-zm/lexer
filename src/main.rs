mod lexer;

fn main() -> miette::Result<()> {
    let input = "let name = 23";

    let mut lexer = lexer::Lexer::initialize(input);
    lexer.next()?;

    Ok(())
}

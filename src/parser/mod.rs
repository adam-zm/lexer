use crate::lexer::Token;

pub struct Parser {
    tokens: Vec<Token>,
}

impl Parser {
    pub fn initilize(tokens: Vec<Token>) -> Parser {
        Parser { tokens: tokens }
    }

    pub fn list(&self) {
        for token in &self.tokens {
            println!("{:?}", token);
        }
    }
}

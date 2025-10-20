use crate::lexer::{Token, TokenType};

pub struct Parser<'a> {
    tokens: &'a Vec<Token>,
    curr_token: Token,
    pos: usize,
}

impl<'a> Parser<'a> {
    pub fn initilize(tokens: &'a Vec<Token>) -> Parser<'a> {
        assert!(tokens.len() > 1);
        Parser {
            tokens: tokens,
            curr_token: tokens[0].clone(),
            pos: 0,
        }
    }

    fn list_tokens(&self) -> String {
        let mut ret = String::new();
        for token in self.tokens {
            ret += (format!("{} ", token.token_t)).as_str();
        }

        ret
    }

    fn advace_token(&mut self) -> Option<Token> {
        if self.pos > self.tokens.len() {
            return None;
        }

        self.pos += 1;
        self.curr_token = self.tokens[self.pos].clone();
        Some(self.curr_token.clone())
    }

    pub fn parse_token(&mut self) -> Result<(), miette::Report> {
        match self.curr_token.token_t {
            TokenType::VAR => {
                let ident = self.advace_token().unwrap();
                if ident.token_t == TokenType::IDENTIFIER {
                    Ok(())
                } else {
                    let mut diagnostic = miette::MietteDiagnostic::new("Unexpected token");
                    //TODO: fix span to match token length
                    diagnostic = diagnostic.with_label(miette::LabeledSpan::at(self.pos, "here"));

                    return Err(miette::Report::with_source_code(
                        diagnostic.into(),
                        self.list_tokens(),
                    ));
                }
            }
            _ => Ok(()),
        }
    }
}

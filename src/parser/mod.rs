use crate::lexer::{Token, TokenType};

#[derive(Debug)]
pub enum Operation {
    CreateVar(String),
    Addition(u32, u32),
}

pub struct Parser<'a> {
    tokens: &'a Vec<Token>,
    ast: &'a mut Vec<Operation>,
    curr_token: Token,
    pos: usize,
}

impl<'a> Parser<'a> {
    pub fn initilize(tokens: &'a Vec<Token>, ast: &'a mut Vec<Operation>) -> Parser<'a> {
        assert!(tokens.len() > 1);
        Parser {
            tokens: tokens,
            ast: ast,
            curr_token: Token {
                token_t: TokenType::EOF,
                value: None,
            },
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

        self.curr_token = self.tokens[self.pos].clone();
        self.pos += 1;
        Some(self.curr_token.clone())
    }

    fn expect_token(&mut self, expected: TokenType) -> bool {
        if self.pos > self.tokens.len() {
            return false;
        }
        let temp = self.tokens[self.pos].clone();

        if temp.token_t == expected {
            return true;
        }
        false
    }

    pub fn parse_token(&mut self) -> Result<(), miette::Report> {
        self.advace_token();
        match self.curr_token.token_t {
            TokenType::VAR => {
                let ident = self.advace_token().unwrap();
                if ident.token_t == TokenType::IDENTIFIER {
                    self.ast.push(Operation::CreateVar(
                        ident.value.expect("Should be a valid identifier"),
                    ));
                    if self.expect_token(TokenType::SEMICOLON) {
                        self.advace_token();
                        Ok(())
                    } else {
                        let diagnostic =
                            miette::MietteDiagnostic::new("Expected semicolon at the end");
                        return Err(miette::Report::with_source_code(
                            diagnostic.into(),
                            self.list_tokens(),
                        ));
                    }
                } else {
                    let mut diagnostic = miette::MietteDiagnostic::new(format!(
                        "Unexpected token {}",
                        ident.token_t
                    ));
                    //TODO: fix span to match token length
                    diagnostic = diagnostic.with_label(miette::LabeledSpan::at(self.pos, "here"));

                    return Err(miette::Report::with_source_code(
                        diagnostic.into(),
                        self.list_tokens(),
                    ));
                }
            }
            TokenType::NUMBER => {
                let number: u32 = self
                    .curr_token
                    .value
                    .as_mut()
                    .unwrap()
                    .clone()
                    .parse()
                    .unwrap();
                if self.expect_token(TokenType::PLUS) {
                    self.advace_token();
                    let mut num_literal = self.advace_token().expect("Second number needed");
                    if num_literal.token_t == TokenType::NUMBER {
                        let num: u32 = num_literal.value.as_mut().unwrap().clone().parse().unwrap();
                        self.ast.push(Operation::Addition(number, num));
                        self.advace_token();
                        Ok(())
                    } else {
                        let diagnostic = miette::MietteDiagnostic::new(format!(
                            "Expected second number but got {}, {}",
                            num_literal.token_t,
                            num_literal.value.unwrap()
                        ));
                        return Err(miette::Report::with_source_code(
                            diagnostic.into(),
                            self.list_tokens(),
                        ));
                    }
                } else {
                    let mut diagnostic = miette::MietteDiagnostic::new(format!(
                        "Unexpected token while parsing addition {}",
                        self.curr_token.token_t
                    ));
                    //TODO: fix span to match token length
                    diagnostic = diagnostic.with_label(miette::LabeledSpan::at(self.pos, "here"));

                    return Err(miette::Report::with_source_code(
                        diagnostic.into(),
                        self.list_tokens(),
                    ));
                }
            }
            _ => {
                let mut diagnostic = miette::MietteDiagnostic::new(format!(
                    "Unexpected token {}",
                    self.curr_token.token_t
                ));
                //TODO: fix span to match token length
                diagnostic = diagnostic.with_label(miette::LabeledSpan::at(self.pos, "here"));

                return Err(miette::Report::with_source_code(
                    diagnostic.into(),
                    self.list_tokens(),
                ));
            }
        }
    }
}

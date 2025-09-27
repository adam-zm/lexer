#[derive(Debug)]
pub enum TokenType {
    SPACE,
    NUM,
    LITERAL,
    IDENT,
    EQUAL,
}

#[derive(Debug)]
pub struct Token {
    token_t: TokenType,
    value: String,
}

pub struct Lexer<'a> {
    input: &'a str,
    rest: &'a str,
}

impl<'a> Lexer<'a> {
    pub fn initialize(input: &'a str) -> Lexer<'a> {
        return Lexer {
            input: input,
            rest: input,
        };
    }

    pub fn next(&mut self) -> Result<Token, miette::MietteDiagnostic> {
        let mut chars = self.rest.chars();
        let ch = chars.next().expect("Should be a char");
        self.rest = chars.as_str();

        match ch {
            ' ' => {
                return Ok(Token {
                    token_t: TokenType::SPACE,
                    value: "".to_string(),
                });
            }
            _ => {
                let offset = self.input.len() - self.rest.len() - ch.len_utf8();
                Err(miette::diagnostic! {
                    labels = vec![miette::LabeledSpan::at_offset(offset, "This char")],
                    "Unexpected token"
                }
                .with_code(self.input.to_string()))
            }
        }
    }
}

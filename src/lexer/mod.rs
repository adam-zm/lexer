#[derive(Debug, PartialEq)]
pub enum TokenType {
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,
    SPACE,
    // One or two character tokens.
    BANG,
    BANG_EQUAL,
    EQUAL,
    EQUAL_EQUAL,
    GREATER,
    GREATER_EQUAL,
    LESS,
    LESS_EQUAL,

    // Literals.
    IDENTIFIER,
    STRING,
    NUMBER,

    // Keywords.
    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,

    EOF,
}

#[derive(Debug)]
pub struct Token {
    pub token_t: TokenType,
    pub value: Option<String>,
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

    fn peek_and_expect(&mut self, expected: char) -> bool {
        if self.rest.is_empty() {
            return false;
        }

        let mut chars = self.rest.chars();
        let ch = chars.next().expect("Should be a char");

        if ch == expected {
            self.rest = chars.as_str();
            return true;
        }

        false
    }

    fn peek_char(&mut self) -> Option<char> {
        if self.rest.is_empty() {
            return None;
        }

        let mut chars = self.rest.chars();
        let ch = chars.next().expect("Should be a char");

        self.rest = chars.as_str();
        Some(ch)
    }

    pub fn next(&mut self) -> Result<Token, miette::Report> {
        if self.rest.is_empty() {
            return Ok(Token {
                token_t: TokenType::EOF,
                value: None,
            });
        }

        let mut chars = self.rest.chars();
        let ch = chars.next().expect("Should be a char");
        self.rest = chars.as_str();

        match ch {
            ' ' => {
                return Ok(Token {
                    token_t: TokenType::SPACE,
                    value: None,
                });
            }
            '*' => {
                return Ok(Token {
                    token_t: TokenType::STAR,
                    value: None,
                });
            }
            '(' => {
                return Ok(Token {
                    token_t: TokenType::LEFT_PAREN,
                    value: None,
                });
            }
            ')' => {
                return Ok(Token {
                    token_t: TokenType::RIGHT_PAREN,
                    value: None,
                });
            }
            '{' => {
                return Ok(Token {
                    token_t: TokenType::LEFT_BRACE,
                    value: None,
                });
            }
            '}' => {
                return Ok(Token {
                    token_t: TokenType::RIGHT_BRACE,
                    value: None,
                });
            }
            ',' => {
                return Ok(Token {
                    token_t: TokenType::COMMA,
                    value: None,
                });
            }
            '.' => {
                return Ok(Token {
                    token_t: TokenType::DOT,
                    value: None,
                });
            }
            ';' => {
                return Ok(Token {
                    token_t: TokenType::SEMICOLON,
                    value: None,
                });
            }
            '-' => {
                return Ok(Token {
                    token_t: TokenType::MINUS,
                    value: None,
                });
            }
            '+' => {
                return Ok(Token {
                    token_t: TokenType::PLUS,
                    value: None,
                });
            }
            '!' => {
                if self.peek_and_expect('=') {
                    return Ok(Token {
                        token_t: TokenType::BANG_EQUAL,
                        value: None,
                    });
                } else {
                    return Ok(Token {
                        token_t: TokenType::BANG,
                        value: None,
                    });
                }
            }
            '=' => {
                if self.peek_and_expect('=') {
                    return Ok(Token {
                        token_t: TokenType::EQUAL_EQUAL,
                        value: None,
                    });
                } else {
                    return Ok(Token {
                        token_t: TokenType::EQUAL,
                        value: None,
                    });
                }
            }
            '>' => {
                if self.peek_and_expect('=') {
                    return Ok(Token {
                        token_t: TokenType::GREATER_EQUAL,
                        value: None,
                    });
                } else {
                    return Ok(Token {
                        token_t: TokenType::GREATER,
                        value: None,
                    });
                }
            }
            '<' => {
                if self.peek_and_expect('=') {
                    return Ok(Token {
                        token_t: TokenType::LESS_EQUAL,
                        value: None,
                    });
                } else {
                    return Ok(Token {
                        token_t: TokenType::LESS,
                        value: None,
                    });
                }
            }
            '/' => {
                if self.peek_and_expect('/') {
                    let offset = self.input.len() - self.rest.len() - ch.len_utf8() - 1;

                    let mut diagnostic =
                        miette::MietteDiagnostic::new("Comments are not supported yet");
                    diagnostic = diagnostic.with_label(miette::LabeledSpan::at(
                        offset..offset + ch.len_utf8() + 1,
                        "Not supported",
                    ));

                    return Err(miette::Report::with_source_code(
                        diagnostic.into(),
                        self.input.to_string(),
                    ));
                } else {
                    return Ok(Token {
                        token_t: TokenType::SLASH,
                        value: None,
                    });
                }
            }
            '"' => {
                let mut value = String::new();
                while let Some(char) = self.peek_char() {
                    if char == '"' {
                        return Ok(Token {
                            token_t: TokenType::STRING,
                            value: Some(value),
                        });
                    }
                    value.push(char);
                }

                let diagnostic = miette::MietteDiagnostic::new("Could not find closing quote");

                return Err(miette::Report::new(diagnostic));
            }
            _ => {
                let offset = self.input.len() - self.rest.len() - ch.len_utf8();

                let mut diagnostic = miette::MietteDiagnostic::new("Unexpected character");
                diagnostic = diagnostic.with_label(miette::LabeledSpan::at(
                    offset..offset + ch.len_utf8(),
                    "This char",
                ));

                return Err(miette::Report::with_source_code(
                    diagnostic.into(),
                    self.input.to_string(),
                ));
            }
        }
    }
}

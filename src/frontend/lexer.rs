use super::token::*;

pub struct Lexer<'a> {
    source: &'a str,

    current: usize,
    start: usize,
    line: usize,
    column: usize,
    start_column: usize,

    pub tokens: Vec<Token>,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            current: 0,
            start: 0,
            line: 0,
            column: 0,
            start_column: 0,
            tokens: Vec::new(),
        }
    }

    pub fn run(&mut self) -> Result<(), String> {
        self.reset();

        loop {
            self.skip_whitespace();
            self.skip_comment();

            let c = match self.peek() {
                Some(c) => c,
                None => break,
            };

            self.start = self.current;
            self.start_column = self.column;

            match c {
                b'(' => {
                    self.advance();
                    self.add_token(TokenType::OpenParen);
                }
                b')' => {
                    self.advance();
                    self.add_token(TokenType::CloseParen);
                }
                b'+' => {
                    self.advance();
                    self.add_token(TokenType::Plus);
                }
                b'-' => {
                    self.advance();
                    self.add_token(TokenType::Minus);
                }
                b'*' => {
                    self.advance();
                    self.add_token(TokenType::Star);
                }
                b'/' => {
                    self.advance();
                    self.add_token(TokenType::Slash);
                }
                b';' => {
                    self.advance();
                    self.add_token(TokenType::Semicolon);
                }
                b'=' => {
                    self.advance();
                    self.add_token(TokenType::Equal);
                }

                _ if c.is_ascii_alphabetic() || c == b'_' => {
                    self.lex_identifier();
                }

                _ if c.is_ascii_digit() => {
                    self.lex_number();
                }

                _ => {
                    return Err(format!("Unknown character: {}", c as char));
                }
            }
        }

        self.tokens.push(Token::new(
            TokenType::Eof,
            self.current,
            0,
            self.line,
            self.column,
        ));

        Ok(())
    }

    fn lex_identifier(&mut self) {
        self.advance();

        while let Some(c) = self.peek() {
            if c.is_ascii_alphanumeric() || c == b'_' {
                self.advance();
            } else {
                break;
            }
        }

        let slice = &self.source[self.start..self.current];

        let kind = self.check_keyword(slice).unwrap_or(TokenType::Identifier);
        self.add_token(kind);
    }

    fn lex_number(&mut self) {
        self.advance();

        while let Some(c) = self.peek() {
            if c.is_ascii_digit() {
                self.advance();
            } else {
                break;
            }
        }

        self.add_token(TokenType::Number);
    }

    fn check_keyword(&self, slice: &str) -> Option<TokenType> {
        match slice {
            "print" => Some(TokenType::Print),
            "int" => Some(TokenType::Int),
            _ => None,
        }
    }

    fn add_token(&mut self, kind: TokenType) {
        self.tokens.push(Token::new(
            kind,
            self.start,
            self.current - self.start,
            self.line,
            self.start_column,
        ));
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            if !c.is_ascii_whitespace() {
                break;
            }
            self.advance();
        }
    }

    fn skip_comment(&mut self) {
        if self.peek() == Some(b'/') && self.peekn(1) == Some(b'/') {
            self.advance();
            self.advance();

            while let Some(c) = self.peek() {
                if c == b'\n' {
                    break;
                }
                self.advance();
            }
        }
    }

    fn peek(&self) -> Option<u8> {
        self.source.as_bytes().get(self.current).copied()
    }

    fn peekn(&self, n: usize) -> Option<u8> {
        self.source.as_bytes().get(self.current + n).copied()
    }

    fn advance(&mut self) -> Option<u8> {
        let c = self.peek()?;
        self.current += 1;

        if c == b'\n' {
            self.line += 1;
            self.column = 0;
        } else {
            self.column += 1;
        }

        Some(c)
    }

    fn reset(&mut self) {
        self.current = 0;
        self.start = 0;
        self.column = 0;
        self.start_column = 0;
        self.line = 0;
        self.tokens.clear();
    }
}
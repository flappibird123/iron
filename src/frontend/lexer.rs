use super::token::*;

pub struct Lexer<'a> {
    source: &'a str,

    current: usize,
    start: usize,
    line: usize,
    column: usize,
    start_column: usize,

    pub tokens: Vec<Token>
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

            tokens: Vec::new()
        }
    }

    pub fn run(&mut self) -> Result<(), String> {
        self.reset();

        while let Some(mut c) = self.peek() {
            loop {
                if self.skip_whitespace() {
                    continue;
                } else if self.skip_comment() {
                    continue;
                } else {
                    break;
                }
            }
            c = self.peek().unwrap();    

            self.start = self.current;
            self.start_column = self.column;

            match c {
                b'(' => {
                    self.advance();
                    self.add_token(TokenType::OpenParen);
                },
                b')' => {
                    self.advance();
                    self.add_token(TokenType::CloseParen);
                },
                b'+' => {
                    self.advance();
                    self.add_token(TokenType::Plus);
                },
                b'-' => {
                    self.advance();
                    self.add_token(TokenType::Minus);
                },
                b'*' => {
                    self.advance();
                    self.add_token(TokenType::Star);
                },
                b'/' => {
                    self.advance();
                    self.add_token(TokenType::Slash);
                },
                b';' => {
                    self.advance();
                    self.add_token(TokenType::Semicolon);
                },
                _ => {
                    if c.is_ascii_alphabetic() || c == b'_' {
                        self.advance();
                        while let Some(c) = self.peek() {
                            if c.is_ascii_alphabetic() || c == b'_' {
                                self.advance();
                            } else {
                                break;
                            }
                        }
                        let slice = &self.source[self.start..self.current];
                        if let Some(kw) = self.check_keyword(slice) {
                            self.add_token(kw);
                        } else {
                            todo!("Implement identifiers");
                        }
                    } else if c.is_ascii_digit() {
                        self.advance();
                        while let Some(c) = self.peek() {
                            if c.is_ascii_digit() {
                                self.advance();
                            } else {
                                break;
                            }
                        }
                        let slice = &self.source[self.start..self.current];
                        self.add_token(TokenType::Integer(slice.parse::<i64>().expect("string should be valid i64")));
                    } else {
                        return Err(format!("Unknown character: {}", c));
                    }
                }
            }
        }    
        Ok(())
    }

    fn check_keyword(&self, slice: &str) -> Option<TokenType> {

        match slice {
            "print" => {
                Some(TokenType::Print)
            },
            _ => {
                None
            }
        }

    }

    fn add_token(&mut self, kind: TokenType) {
        self.tokens.push(Token::new(kind, self.start, self.current - self.start, self.line, self.start_column));
    }

    fn skip_comment(&mut self) -> bool {
        let mut modified = false;
        if self.peek().unwrap_or_default() == '/' as u8 && self.peekn(1).unwrap_or_default() == '/' as u8 {
            modified = true;
            self.advance();
            self.advance();
            while let Some(c) = self.peek() {
                if c == '\n' as u8 {
                    break;
                }
                self.advance();
            }
        }
        modified
    }

    fn skip_whitespace(&mut self) -> bool {
        let mut modified = false;
        while let Some(c) = self.peek() { 
            if !c.is_ascii_whitespace() {
                break;
            }
            self.advance();
            modified = true;
        }
        modified
    }

    fn peek(&self) -> Option<u8> {
        self.source.as_bytes().get(self.current).copied()
    }

    fn peekn(&self, index: usize) -> Option<u8> {
        self.source.as_bytes().get(self.current + index).copied()
    }

    fn advance(&mut self) -> Option<u8> {
        if let Some(c) = self.peek() {
            self.current += 1;
            if c == '\n' as u8 {
                self.line += 1;
                self.column = 0;
            } else {
                self.column += 1;
            }
            Some(c)
        } else {
            None
        }
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
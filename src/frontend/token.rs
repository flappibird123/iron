
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum TokenType {
    Integer(i64),

    Print,

    OpenParen,
    CloseParen,

    Plus,
    Minus,
    Star,
    Slash,

    Semicolon,

    Eof
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Token {
    pub kind: TokenType,
    offset: usize,
    length: usize,
    line: usize,
    column: usize
}

impl Token {
    pub fn new(kind: TokenType, offset: usize, length: usize, line: usize, column: usize) -> Self {
        Self {
            kind,
            offset,
            length,
            line,
            column
        }
    }
}
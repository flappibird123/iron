
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum TokenType {
    IntegerLiteral(i64),

    // types
    Int,

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
    pub line: usize,
    pub column: usize
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
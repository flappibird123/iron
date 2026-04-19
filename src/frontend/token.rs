
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum TokenType {
    Number,

    // types
    Int,

    Print,

    Identifier,

    Equal,

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
    pub offset: usize,
    pub length: usize,
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
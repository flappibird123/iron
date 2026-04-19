use super::lexer;
use super::ast::*;
use super::token::{Token, TokenType};
use owo_colors::OwoColorize;

pub struct Parser<'a> {
    current: usize,
    tokens: Vec<Token>,
    source: &'a str
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str) -> Result<Self, String> {
        let mut lexer = lexer::Lexer::new(source);
        lexer.run()?;
        Ok(Self {
            tokens: lexer.tokens,
            current: 0,
            source
        })
    }

    pub fn run(&mut self) -> Result<Program, String> {
        let mut program: Program = Program::new();

        while !self.is_at_end() {
            program.statements.push(self.parse_stmt()?); 
        }
        Ok(program)
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    fn advance(&mut self) -> Option<Token> {
        let tok = self.tokens.get(self.current)?.clone();
        self.current += 1;
        Some(tok)
    }

    fn is_at_end(&self) -> bool {
        matches!(self.peek().map(|t| &t.kind), Some(TokenType::Eof) | None)
    }

    fn parse_factor(&mut self) -> Result<Expr, String> {
        let token = self.advance().ok_or("Unexpected EOF")?;

        match &token.kind {
            TokenType::Number => {
                let n = &self.source[token.offset..token.offset + token.length]; 
                Ok(Expr::IntegerLiteral(n.parse::<i64>().expect("NaN")))
            },

            TokenType::OpenParen => {
                let expr = self.parse_expr()?;

                match self.peek() {
                    Some(t) if t.kind == TokenType::CloseParen => {
                        self.advance();
                        Ok(expr)
                    }
                    _ => Err("Expected ')'".into()),
                }
            }

            _ => Err("Expected number or '('".into()),
        }
    }

    fn parse_stmt(&mut self) -> Result<Stmt, String> {
        let stmt = match &self.peek().ok_or("Unexpected end of input")?.kind {
            TokenType::Print => {
                self.advance(); // consume 'print'
                let expr = self.parse_expr()?;
                Stmt::Print(expr)
            }
            _ => return Err("Expected statement".into()),
        };

        match self.peek() {
            Some(t) if t.kind == TokenType::Semicolon => {
                self.advance();
            }
            other => {
                match other {
                    Some(tok)=> {
                        return Err(format!("{}:{} {} {}", tok.line + 1, tok.column + 1, "error: ".bright_red().bold(), "Expected ';' after statement"))
                    },
                    None => {
                        return Err("Expected ';' after statement".into());
                    }
                }
            }
        }

        Ok(stmt)
    }

    fn parse_term(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_factor()?;

        loop {
            match self.peek().map(|t| &t.kind) {
                Some(TokenType::Star) => {
                    self.advance();
                    let right = self.parse_factor()?;
                    expr = Expr::Binary {
                        left: Box::new(expr),
                        op: BinaryOp::Mul,
                        right: Box::new(right),
                    };
                }

                Some(TokenType::Slash) => {
                    self.advance();
                    let right = self.parse_factor()?;
                    expr = Expr::Binary {
                        left: Box::new(expr),
                        op: BinaryOp::Div,
                        right: Box::new(right),
                    };
                }

                _ => break,
            }
        }

        Ok(expr)
    }

    fn parse_expr(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_term()?;

        loop {
            match self.peek().map(|t| &t.kind) {
                Some(TokenType::Plus) => {
                    self.advance();
                    let right = self.parse_term()?;
                    expr = Expr::Binary {
                        left: Box::new(expr),
                        op: BinaryOp::Add,
                        right: Box::new(right),
                    };
                }

                Some(TokenType::Minus) => {
                    self.advance();
                    let right = self.parse_term()?;
                    expr = Expr::Binary {
                        left: Box::new(expr),
                        op: BinaryOp::Sub,
                        right: Box::new(right),
                    };
                }

                _ => break,
            }
        }

        Ok(expr)
    }
}
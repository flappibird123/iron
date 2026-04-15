use super::lexer;
use super::ast::*;
use super::token::{Token, TokenType};

pub struct Parser {
    current: usize,
    tokens: Vec<Token>
}

impl Parser {
    pub fn new(source: &str) -> Result<Self, String> {
        let mut lexer = lexer::Lexer::new(source);
        lexer.run()?;
        Ok(Self {
            tokens: lexer.tokens,
            current: 0
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

    fn advance(&mut self) -> Option<&Token> {
        let tok = self.tokens.get(self.current)?;
        self.current += 1;
        Some(tok)
    }

    fn is_at_end(&self) -> bool {
        matches!(self.peek().map(|t| &t.kind), Some(TokenType::Eof) | None)
    }

    fn parse_factor(&mut self) -> Result<Expr, String> {
        let token = self.advance().ok_or("Unexpected EOF")?;

        match &token.kind {
            TokenType::Integer(n) => Ok(Expr::IntegerLiteral(*n)),

            TokenType::OpenParen => {
                let expr = self.parse_expr()?;

                match self.peek() {
                    Some(t) if t.kind == TokenType::OpenParen => {
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
        match self.peek().ok_or("Unexpected end of input")?.kind {
            TokenType::Print => {
                self.advance(); // consume 'print'

                let expr = self.parse_expr()?;

                Ok(Stmt::Print(expr))
            }

            _ => Err("Expected statement".into()),
        }
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
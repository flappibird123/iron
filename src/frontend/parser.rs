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
            TokenType::Identifier => {
                let name = self.source[token.offset..token.offset + token.length].to_string();
                Ok(Expr::Identifier(name))
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
            },
            TokenType::Int => {
                self.parse_var_decl()?
            },
            _ => return Err("Expected statement".into()),
        };

        match self.peek() {
            Some(t) if t.kind == TokenType::Semicolon => {
                self.advance();
            }
            other => {
                match other {
                    Some(tok)=> {
                        return Err(self.err(tok, "expected ; after statement"));
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

    fn parse_var_decl(&mut self) -> Result<Stmt, String> {
        let tmp = self.advance().ok_or("idk what happened")?;
        let name_token = self.advance().ok_or(self.err(&tmp, "expected identifier"))?;

        if name_token.kind != TokenType::Identifier {
            return Err(self.err(&name_token,"expected identifier"));
        }

        let name = self.source[name_token.offset..name_token.offset + name_token.length].to_string();

        match self.peek() {
            Some(t) if t.kind == TokenType::Equal => {
                self.advance();
            },
            other => {
                match other {
                    Some(t) => return Err(self.err(t, "expected '=' after identifier")),
                    None => return Err("Unexpected end of input".into())
                }
            }
        }

        let value = self.parse_expr()?;
        Ok(Stmt::VarDecl {
            name,
            ty: Type::Int,
            value,
        })
    }

    fn err(&self, token: &Token, msg: &str) -> String {
        format!("{}:{} {} {}", token.line + 1, token.column + 1, "error: ".bright_red().bold(), msg)
    }
}
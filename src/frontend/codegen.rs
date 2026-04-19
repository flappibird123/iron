use super::chunk::Chunk;
use super::parser::Parser;
use super::ast::{Stmt, Expr, BinaryOp};
use crate::runtime::opcodes::OpCode;

pub struct Compiler<'a> {
    bytecode: Vec<u8>,
    constants: Vec<i64>,
    parser: Parser<'a>,
}

impl<'a> Compiler<'a> {
    pub fn new(source: &'a str) -> Result<Self, String> {
        let parser = Parser::new(source)?; 
        Ok(Self {
            bytecode: Vec::new(),
            constants: Vec::new(),
            parser
        })
    }

    pub fn compile(mut self) -> Result<Chunk, String> {
        let program = self.parser.run()?;

        for stmt in program.statements {
            self.compile_stmt(stmt);
        }

        self.emit(OpCode::Hlt);

        Ok(self.into_chunk())        
    }

    fn into_chunk(self) -> Chunk {
        Chunk::new(self.bytecode, self.constants)
    }
    fn compile_stmt(&mut self, stmt: Stmt) {
        match stmt {
            Stmt::Print(expr) => {
                self.compile_expr(expr);
                self.emit(OpCode::Print);
            },
            Stmt::ExprStmt(expr) => {
                self.compile_expr(expr);
                // discard the result
            }
        }
    }

    fn compile_expr(&mut self, expr: Expr) {
        match expr {
            Expr::IntegerLiteral(n) => {
                self.emit_const(n);
            },
            Expr::Binary { left, op, right } => {
                self.compile_expr(*left);
                self.compile_expr(*right);
                match op {
                    BinaryOp::Add => self.emit(OpCode::Add),
                    BinaryOp::Sub => self.emit(OpCode::Sub),
                    BinaryOp::Mul => self.emit(OpCode::Mul),
                    BinaryOp::Div => self.emit(OpCode::Div),
                }
            }
        }
    }

    fn emit_const(&mut self, constant: i64) {
        let i = self.constants.len();
        self.constants.push(constant);
        self.emit(OpCode::LoadConst);
        self.bytecode.push(i as u8);
    }

    fn emit(&mut self, opcode: OpCode) {
        self.bytecode.push(opcode as u8);
    }
}
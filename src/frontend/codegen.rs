use std::collections::HashMap;

use super::chunk::Chunk;
use super::ast::{Stmt, Expr, BinaryOp};
use crate::runtime::opcodes::OpCode;
use super::parser::Parser;

pub struct Compiler {
    bytecode: Vec<u8>,
    constants: Vec<i64>,

    // local variable storage (name → slot index)
    locals: HashMap<String, u8>,
    next_local: u8,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            bytecode: Vec::new(),
            constants: Vec::new(),
            locals: HashMap::new(),
            next_local: 0,
        }
    }

    // Entry point: AST → bytecode
    pub fn compile(mut self, source: &str) -> Result<Chunk, String> {
        let mut parser = Parser::new(source)?;
        let program = parser.run()?;
        for stmt in program.statements {
            self.compile_stmt(stmt);
        }

        self.emit(OpCode::Hlt);
        Ok(self.into_chunk())
    }

    fn into_chunk(self) -> Chunk {
        Chunk::new(self.bytecode, self.constants)
    }

    // -------------------------
    // Statements
    // -------------------------
    fn compile_stmt(&mut self, stmt: Stmt) {
        match stmt {
            Stmt::Print(expr) => {
                self.compile_expr(expr);
                self.emit(OpCode::Print);
            }

            Stmt::ExprStmt(expr) => {
                self.compile_expr(expr);
            }

            Stmt::VarDecl { name, ty: _, value } => {
                self.compile_expr(value);

                let slot = self.get_or_create_local(name);

                self.emit(OpCode::StoreLocal);
                self.bytecode.push(slot);
            }
        }
    }

    // -------------------------
    // Expressions
    // -------------------------
    fn compile_expr(&mut self, expr: Expr) {
        match expr {
            Expr::IntegerLiteral(n) => {
                self.emit_const(n);
            }

            Expr::Identifier(name) => {
                let slot = *self.locals.get(&name)
                    .expect("Semantic pass guarantees variable exists");

                self.emit(OpCode::LoadLocal);
                self.bytecode.push(slot);
            }

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

    // -------------------------
    // Locals (symbol → slot)
    // -------------------------
    fn get_or_create_local(&mut self, name: String) -> u8 {
        if let Some(&slot) = self.locals.get(&name) {
            return slot;
        }

        let slot = self.next_local;
        self.locals.insert(name, slot);
        self.next_local += 1;

        slot
    }

    // -------------------------
    // Constants
    // -------------------------
    fn emit_const(&mut self, constant: i64) {
        let index = self.constants.len();
        self.constants.push(constant);

        self.emit(OpCode::LoadConst);
        self.bytecode.push(index as u8);
    }

    // -------------------------
    // Bytecode emission
    // -------------------------
    fn emit(&mut self, opcode: OpCode) {
        self.bytecode.push(opcode as u8);
    }
}
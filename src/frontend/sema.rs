use std::collections::HashMap;


use super::ast::*;

pub struct SemanticAnalyzer {
    symbols: HashMap<String, Type>
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        Self {
            symbols: HashMap::new(),
        }
    }

    pub fn analyze_program(&mut self, program: &Program) -> Result<(), String> {
        for stmt in &program.statements {
            self.analyze_stmt(stmt)?;
        }
        Ok(())
    }

    fn analyze_stmt(&mut self, stmt: &Stmt) -> Result<(), String> {
        match stmt {
            Stmt::Print(expr) => {
                self.analyze_expr(expr)?;
            }

            Stmt::ExprStmt(expr) => {
                self.analyze_expr(expr)?;
            }

            Stmt::VarDecl { name, ty, value } => {
                let value_type = self.analyze_expr(value)?;

                // type check
                if *ty != value_type {
                    return Err(format!(
                        "Type mismatch: expected {:?}, got {:?}",
                        ty, value_type
                    ));
                }

                // insert into symbol table
                self.symbols.insert(name.clone(), ty.clone());
            }
        }

        Ok(())
    }

    fn analyze_expr(&mut self, expr: &Expr) -> Result<Type, String> {
        match expr {
            Expr::IntegerLiteral(_) => Ok(Type::Int),

            Expr::Identifier(name) => {
                self.symbols
                    .get(name)
                    .cloned()
                    .ok_or(format!("Undefined variable: {}", name))
            }

            Expr::Binary { left, op: _, right } => {
                let lt = self.analyze_expr(left)?;
                let rt = self.analyze_expr(right)?;

                if lt != Type::Int || rt != Type::Int {
                    return Err("Binary operations only supported on ints".into());
                }

                Ok(Type::Int)
            }
        }
    }
}
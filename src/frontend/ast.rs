

#[derive(Debug)]
pub enum Stmt {
    Print(Expr),
    ExprStmt(Expr),
    VarDecl {
        name: String,
        ty: Type,
        value: Expr
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    Int
}

#[derive(Debug)]
pub enum Expr {
    IntegerLiteral(i64),
    Binary {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>
    },
    Identifier(String)
}

#[derive(Debug)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div
}

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Stmt>
}

impl Program {
    pub fn new() -> Self {
        Self {
            statements: Vec::new()
        }
    }
}
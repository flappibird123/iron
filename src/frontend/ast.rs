

#[derive(Debug)]
pub enum Stmt {
    Print(Expr),
    ExprStmt(Expr),
}

#[derive(Debug)]
pub enum Expr {
    IntegerLiteral(i64),
    Binary{
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>
    }
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
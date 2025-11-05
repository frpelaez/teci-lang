use crate::error::*;
use crate::expr::*;

#[derive(Clone)]
pub enum Stmt {
    Expression(ExpressionStmt),
    Print(PrintStmt),
}

impl Stmt {
    pub fn accept<T>(&self, visitor: &dyn StmtVisitor<T>) -> Result<T, TeciError> {
        match self {
            Stmt::Expression(exp) => exp.accept(visitor),
            Stmt::Print(exp) => exp.accept(visitor),
        }
    }
}

#[derive(Clone)]
pub struct ExpressionStmt {
    pub expression: Expr,
}

#[derive(Clone)]
pub struct PrintStmt {
    pub expression: Expr,
}

pub trait StmtVisitor<T> {
    fn visit_expression_stmt(&self, expr: &ExpressionStmt) -> Result<T, TeciError>;
    fn visit_print_stmt(&self, expr: &PrintStmt) -> Result<T, TeciError>;
}

impl ExpressionStmt {
    pub fn accept<T>(&self, visitor: &dyn StmtVisitor<T>) -> Result<T, TeciError> {
        visitor.visit_expression_stmt(self)
    }
}

impl PrintStmt {
    pub fn accept<T>(&self, visitor: &dyn StmtVisitor<T>) -> Result<T, TeciError> {
        visitor.visit_print_stmt(self)
    }
}

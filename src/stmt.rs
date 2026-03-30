use crate::error::*;
use crate::expr::*;
use crate::token::*;

#[derive(Clone)]
pub enum Stmt {
    Expression(ExpressionStmt),
    Print(PrintStmt),
    Let(LetStmt),
}

impl Stmt {
    pub fn accept<T>(&self, visitor: &dyn StmtVisitor<T>) -> Result<T, TeciError> {
        match self {
            Stmt::Expression(exp) => exp.accept(visitor),
            Stmt::Print(exp) => exp.accept(visitor),
            Stmt::Let(exp) => exp.accept(visitor),
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

#[derive(Clone)]
pub struct LetStmt {
    pub name: Token,
    pub initializer: Option<Expr>,
}

pub trait StmtVisitor<T> {
    fn visit_expression_stmt(&self, stmt: &ExpressionStmt) -> Result<T, TeciError>;
    fn visit_print_stmt(&self, stmt: &PrintStmt) -> Result<T, TeciError>;
    fn visit_let_stmt(&self, stmt: &LetStmt) -> Result<T, TeciError>;
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

impl LetStmt {
    pub fn accept<T>(&self, visitor: &dyn StmtVisitor<T>) -> Result<T, TeciError> {
        visitor.visit_let_stmt(self)
    }
}

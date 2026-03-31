use crate::error::*;
use crate::expr::*;
use crate::token::*;

#[derive(Clone)]
pub enum Stmt {
    Break(BreakStmt),
    Block(BlockStmt),
    If(IfStmt),
    Expression(ExpressionStmt),
    Print(PrintStmt),
    Let(LetStmt),
    While(WhileStmt),
}

impl Stmt {
    pub fn accept<T>(&self, visitor: &dyn StmtVisitor<T>) -> Result<T, TeciResult> {
        match self {
            Stmt::Break(exp) => exp.accept(visitor),
            Stmt::Block(exp) => exp.accept(visitor),
            Stmt::If(exp) => exp.accept(visitor),
            Stmt::Expression(exp) => exp.accept(visitor),
            Stmt::Print(exp) => exp.accept(visitor),
            Stmt::Let(exp) => exp.accept(visitor),
            Stmt::While(exp) => exp.accept(visitor),
        }
    }
}

#[derive(Clone)]
pub struct BreakStmt {
    pub token: Token,
}

#[derive(Clone)]
pub struct BlockStmt {
    pub statements: Vec<Stmt>,
}

#[derive(Clone)]
pub struct IfStmt {
    pub condition: Expr,
    pub then_branch: Box<Stmt>,
    pub else_branch: Option<Box<Stmt>>,
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

#[derive(Clone)]
pub struct WhileStmt {
    pub condition: Expr,
    pub body: Box<Stmt>,
}

pub trait StmtVisitor<T> {
    fn visit_break_stmt(&self, stmt: &BreakStmt) -> Result<T, TeciResult>;
    fn visit_block_stmt(&self, stmt: &BlockStmt) -> Result<T, TeciResult>;
    fn visit_if_stmt(&self, stmt: &IfStmt) -> Result<T, TeciResult>;
    fn visit_expression_stmt(&self, stmt: &ExpressionStmt) -> Result<T, TeciResult>;
    fn visit_print_stmt(&self, stmt: &PrintStmt) -> Result<T, TeciResult>;
    fn visit_let_stmt(&self, stmt: &LetStmt) -> Result<T, TeciResult>;
    fn visit_while_stmt(&self, stmt: &WhileStmt) -> Result<T, TeciResult>;
}

impl BreakStmt {
    pub fn accept<T>(&self, visitor: &dyn StmtVisitor<T>) -> Result<T, TeciResult> {
        visitor.visit_break_stmt(self)
    }
}

impl BlockStmt {
    pub fn accept<T>(&self, visitor: &dyn StmtVisitor<T>) -> Result<T, TeciResult> {
        visitor.visit_block_stmt(self)
    }
}

impl IfStmt {
    pub fn accept<T>(&self, visitor: &dyn StmtVisitor<T>) -> Result<T, TeciResult> {
        visitor.visit_if_stmt(self)
    }
}

impl ExpressionStmt {
    pub fn accept<T>(&self, visitor: &dyn StmtVisitor<T>) -> Result<T, TeciResult> {
        visitor.visit_expression_stmt(self)
    }
}

impl PrintStmt {
    pub fn accept<T>(&self, visitor: &dyn StmtVisitor<T>) -> Result<T, TeciResult> {
        visitor.visit_print_stmt(self)
    }
}

impl LetStmt {
    pub fn accept<T>(&self, visitor: &dyn StmtVisitor<T>) -> Result<T, TeciResult> {
        visitor.visit_let_stmt(self)
    }
}

impl WhileStmt {
    pub fn accept<T>(&self, visitor: &dyn StmtVisitor<T>) -> Result<T, TeciResult> {
        visitor.visit_while_stmt(self)
    }
}

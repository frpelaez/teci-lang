use crate::token::*;
use crate::object::*;
use crate::error::*;

#[derive(Clone)]
pub enum Expr {
    Binary(BinaryExpr),
    Grouping(GroupingExpr),
    Literal(LiteralExpr),
    Unary(UnaryExpr),
}

impl Expr {
    pub fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> Result<T, TeciError> {
        match self {
            Expr::Binary(exp) => exp.accept(visitor),
            Expr::Grouping(exp) => exp.accept(visitor),
            Expr::Literal(exp) => exp.accept(visitor),
            Expr::Unary(exp) => exp.accept(visitor),
        }
    }
}

#[derive(Clone)]
pub struct BinaryExpr {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

#[derive(Clone)]
pub struct GroupingExpr {
    pub expression: Box<Expr>,
}

#[derive(Clone)]
pub struct LiteralExpr {
    pub value: Option<Object>,
}

#[derive(Clone)]
pub struct UnaryExpr {
    pub operator: Token,
    pub right: Box<Expr>,
}

pub trait ExprVisitor<T> {
    fn visit_binary_expr(&self, expr: &BinaryExpr) -> Result<T, TeciError>;
    fn visit_grouping_expr(&self, expr: &GroupingExpr) -> Result<T, TeciError>;
    fn visit_literal_expr(&self, expr: &LiteralExpr) -> Result<T, TeciError>;
    fn visit_unary_expr(&self, expr: &UnaryExpr) -> Result<T, TeciError>;
}

impl BinaryExpr {
    pub fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> Result<T, TeciError> {
        visitor.visit_binary_expr(self)
    }
}

impl GroupingExpr {
    pub fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> Result<T, TeciError> {
        visitor.visit_grouping_expr(self)
    }
}

impl LiteralExpr {
    pub fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> Result<T, TeciError> {
        visitor.visit_literal_expr(self)
    }
}

impl UnaryExpr {
    pub fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> Result<T, TeciError> {
        visitor.visit_unary_expr(self)
    }
}

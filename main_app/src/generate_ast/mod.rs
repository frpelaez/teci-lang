use crate::{
    error::TeciError,
    token::{Object, Token},
};

pub enum Expr {
    Unary(UnaryExpr),
    Binary(BinaryExpr),
    Grouping(GroupingExpr),
    Literal(LiteralExpr),
}

pub struct UnaryExpr {
    operator: Token,
    right: Box<Expr>,
}

pub struct BinaryExpr {
    left: Box<Expr>,
    operator: Token,
    right: Box<Expr>,
}

pub struct GroupingExpr {
    expression: Box<Expr>,
}

pub struct LiteralExpr {
    value: Object,
}

pub trait ExprVisitor<T> {
    fn visit_unary_expr(&self, expr: &UnaryExpr) -> Result<T, TeciError>;
    fn visit_binary_expr(&self, expr: &BinaryExpr) -> Result<T, TeciError>;
    fn visit_grouping_expr(&self, expr: &GroupingExpr) -> Result<T, TeciError>;
    fn visit_literal_expr(&self, expr: &LiteralExpr) -> Result<T, TeciError>;
}

impl UnaryExpr {
    fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> Result<T, TeciError> {
        visitor.visit_unary_expr(self)
    }
}

impl BinaryExpr {
    fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> Result<T, TeciError> {
        visitor.visit_binary_expr(self)
    }
}

impl GroupingExpr {
    fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> Result<T, TeciError> {
        visitor.visit_grouping_expr(self)
    }
}

impl LiteralExpr {
    fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> Result<T, TeciError> {
        visitor.visit_literal_expr(self)
    }
}

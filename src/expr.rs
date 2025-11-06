use crate::token::*;
use crate::object::*;
use crate::error::*;

#[derive(Clone)]
pub enum Expr {
    Assign(AssignExpr),
    Binary(BinaryExpr),
    Grouping(GroupingExpr),
    Literal(LiteralExpr),
    Unary(UnaryExpr),
    Variable(VariableExpr),
}

impl Expr {
    pub fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> Result<T, TeciError> {
        match self {
            Expr::Assign(exp) => exp.accept(visitor),
            Expr::Binary(exp) => exp.accept(visitor),
            Expr::Grouping(exp) => exp.accept(visitor),
            Expr::Literal(exp) => exp.accept(visitor),
            Expr::Unary(exp) => exp.accept(visitor),
            Expr::Variable(exp) => exp.accept(visitor),
        }
    }
}

#[derive(Clone)]
pub struct AssignExpr {
    pub name: Token,
    pub value: Box<Expr>,
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

#[derive(Clone)]
pub struct VariableExpr {
    pub name: Token,
}

pub trait ExprVisitor<T> {
    fn visit_assign_expr(&self, expr: &AssignExpr) -> Result<T, TeciError>;
    fn visit_binary_expr(&self, expr: &BinaryExpr) -> Result<T, TeciError>;
    fn visit_grouping_expr(&self, expr: &GroupingExpr) -> Result<T, TeciError>;
    fn visit_literal_expr(&self, expr: &LiteralExpr) -> Result<T, TeciError>;
    fn visit_unary_expr(&self, expr: &UnaryExpr) -> Result<T, TeciError>;
    fn visit_variable_expr(&self, expr: &VariableExpr) -> Result<T, TeciError>;
}

impl AssignExpr {
    pub fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> Result<T, TeciError> {
        visitor.visit_assign_expr(self)
    }
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

impl VariableExpr {
    pub fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> Result<T, TeciError> {
        visitor.visit_variable_expr(self)
    }
}

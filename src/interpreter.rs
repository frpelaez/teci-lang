use crate::error::TeciError;
use crate::expr::*;
use crate::object::Object;
use crate::token_type::TokenType;

pub struct Interpreter;

impl Interpreter {
    fn evaluate(&self, expr: &Expr) -> Result<Object, TeciError> {
        expr.accept(self)
    }

    fn is_truthy(obj: &Object) -> bool {
        match obj {
            Object::Num(x) => *x != 0.0,
            Object::Str(s) => !s.is_empty(),
            Object::True => true,
            Object::False => false,
            Object::Nil => false,
        }
    }
}

impl ExprVisitor<Object> for Interpreter {
    fn visit_unary_expr(&self, expr: &UnaryExpr) -> Result<Object, TeciError> {
        let right = self.evaluate(&expr.right)?;
        match expr.operator.ttype {
            TokenType::Minus => {
                return if let Object::Num(x) = right {
                    Ok(Object::Num(-x))
                } else {
                    Ok(Object::Nil)
                };
            }
            TokenType::Bang => {
                return if Interpreter::is_truthy(&right) {
                    Ok(Object::False)
                } else {
                    Ok(Object::True)
                };
            }
            _ => {}
        }
        Err(TeciError::new(expr.operator.line, "Unreachable ???"))
    }

    fn visit_binary_expr(&self, expr: &BinaryExpr) -> Result<Object, TeciError> {
        todo!()
    }

    fn visit_grouping_expr(&self, expr: &GroupingExpr) -> Result<Object, TeciError> {
        self.evaluate(&expr.expression)
    }

    fn visit_literal_expr(&self, expr: &LiteralExpr) -> Result<Object, TeciError> {
        Ok(expr.value.clone().unwrap())
    }
}

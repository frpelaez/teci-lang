use crate::{error::TeciError, expr::*};

pub struct AstPrinter;

impl AstPrinter {
    pub fn print(&self, expr: &Expr) -> Result<String, TeciError> {
        expr.accept(self)
    }

    fn parenthesize(&self, lex: &String, exprs: &[Box<Expr>]) -> Result<String, TeciError> {
        let mut builder = format!("({lex}");
        for expr in exprs {
            builder = format!("{builder} {}", expr.accept(self)?);
        }
        builder = format!("{builder})");
        Ok(builder)
    }
}

impl ExprVisitor<String> for AstPrinter {
    fn visit_binary_expr(&self, expr: &BinaryExpr) -> Result<String, TeciError> {
        self.parenthesize(
            &expr.operator.lexeme,
            &[expr.left.clone(), expr.right.clone()],
        )
    }
    fn visit_unary_expr(&self, expr: &UnaryExpr) -> Result<String, TeciError> {
        self.parenthesize(&expr.operator.lexeme, &[expr.right.clone()])
    }
    fn visit_grouping_expr(&self, expr: &GroupingExpr) -> Result<String, TeciError> {
        self.parenthesize(&"group".to_string(), &[expr.expression.clone()])
    }
    fn visit_literal_expr(&self, expr: &LiteralExpr) -> Result<String, TeciError> {
        if let Some(val) = &expr.value {
            Ok(val.to_string())
        } else {
            Ok("nil".to_string())
        }
    }
}

// use crate::{error::TeciError, expr::*};
//
// struct AstPrinter;
//
// impl AstPrinter {
//     fn print(&self, expr: &Expr) -> Result<String, TeciError> {
//         expr.accept(self)
//     }
//
//     fn parenthesize(&self, lex: String, exprs: Vec<Expr>) -> Result<String, TeciError> {}
// }
//
// impl ExprVisitor<String> for AstPrinter {
//     fn visit_binary_expr(&self, expr: &BinaryExpr) -> Result<String, TeciError> {
//         self.parenthesize(expr.operator.lexeme, [expr.left, expr.right])
//     }
//     fn visit_unary_expr(&self, expr: &UnaryExpr) -> Result<String, TeciError> {
//         todo!()
//     }
//     fn visit_grouping_expr(&self, expr: &GroupingExpr) -> Result<String, TeciError> {
//         todo!()
//     }
//     fn visit_literal_expr(&self, expr: &LiteralExpr) -> Result<String, TeciError> {
//         todo!()
//     }
// }

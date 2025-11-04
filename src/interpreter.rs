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
            Object::Bool(b) => *b,
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
                    Err(TeciError::new(
                        expr.operator.line,
                        "Negate operator '-' is not valid for non numeric operand",
                    ))
                };
            }
            TokenType::Bang => return Ok(Object::Bool(!Interpreter::is_truthy(&right))),
            _ => {}
        }
        Err(TeciError::new(expr.operator.line, "Unreachable ???"))
    }

    fn visit_binary_expr(&self, expr: &BinaryExpr) -> Result<Object, TeciError> {
        let left = self.evaluate(&expr.left)?;
        let right = self.evaluate(&expr.right)?;
        match expr.operator.ttype {
            TokenType::Minus => {
                if let (Object::Num(a), Object::Num(b)) = (&left, &right) {
                    Ok(Object::Num(a - b))
                } else {
                    Err(TeciError::new(
                        expr.operator.line,
                        "Invalid operator '-' for non numeric",
                    ))
                }
            }
            TokenType::Star => {
                if let (Object::Num(a), Object::Num(b)) = (&left, &right) {
                    Ok(Object::Num(a * b))
                } else {
                    Err(TeciError::new(
                        expr.operator.line,
                        "Invalid operator '*' for non numeric operands",
                    ))
                }
            }
            TokenType::Slash => {
                if let (Object::Num(a), Object::Num(b)) = (&left, &right) {
                    Ok(Object::Num(a / b))
                } else {
                    Err(TeciError::new(
                        expr.operator.line,
                        "Invalid operator '/' for non numeric operands",
                    ))
                }
            }
            TokenType::Plus => {
                if let (Object::Num(a), Object::Num(b)) = (&left, &right) {
                    Ok(Object::Num(a + b))
                } else if let (Object::Str(a), Object::Str(b)) = (left, right) {
                    Ok(Object::Str(format!("{a}{b}")))
                } else {
                    Err(TeciError::new(
                        expr.operator.line,
                        "Invalid operator '+' for non numeric and non string operands",
                    ))
                }
            }
            _ => Err(TeciError::new(expr.operator.line, "Unimplemented (yet)")),
        }
    }

    fn visit_grouping_expr(&self, expr: &GroupingExpr) -> Result<Object, TeciError> {
        self.evaluate(&expr.expression)
    }

    fn visit_literal_expr(&self, expr: &LiteralExpr) -> Result<Object, TeciError> {
        Ok(expr.value.clone().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::*;

    #[test]
    fn t_unary_minus() {
        let interpreter = Interpreter {};
        let unary = UnaryExpr {
            operator: Token::new(TokenType::Minus, "-".to_string(), None, 0),
            right: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Object::Num(69f64)),
            })),
        };
        assert_eq!(
            Object::Num(-69f64),
            interpreter.visit_unary_expr(&unary).unwrap()
        );
    }

    #[test]
    fn t_unary_bang() {
        let interpreter = Interpreter {};
        let boolean = UnaryExpr {
            operator: Token::new(TokenType::Bang, "!".to_string(), None, 0),
            right: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Object::Bool(true)),
            })),
        };
        assert_eq!(
            Object::Bool(false),
            interpreter.visit_unary_expr(&boolean).unwrap()
        );
    }

    #[test]
    fn t_substraction() {
        let interpreter = Interpreter {};
        let expr = BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Object::Num(70.0)),
            })),
            operator: Token::new(TokenType::Minus, "-".to_string(), None, 0),
            right: Box::new(Expr::Unary(UnaryExpr {
                operator: Token::new(TokenType::Minus, "-".to_string(), None, 0),
                right: Box::new(Expr::Literal(LiteralExpr {
                    value: Some(Object::Num(1.0)),
                })),
            })),
        };
        assert_eq!(
            Object::Num(71.0),
            interpreter.visit_binary_expr(&expr).unwrap()
        );
    }

    #[test]
    fn t_mulitplication() {
        let interpreter = Interpreter {};
        let expr = BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Object::Num(70.0)),
            })),
            operator: Token::new(TokenType::Star, "*".to_string(), None, 0),
            right: Box::new(Expr::Unary(UnaryExpr {
                operator: Token::new(TokenType::Minus, "-".to_string(), None, 0),
                right: Box::new(Expr::Literal(LiteralExpr {
                    value: Some(Object::Num(2.0)),
                })),
            })),
        };
        assert_eq!(
            Object::Num(-140.0),
            interpreter.visit_binary_expr(&expr).unwrap()
        );
    }

    #[test]
    fn t_division() {
        let interpreter = Interpreter {};
        let expr = BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Object::Num(70.0)),
            })),
            operator: Token::new(TokenType::Slash, "/".to_string(), None, 0),
            right: Box::new(Expr::Unary(UnaryExpr {
                operator: Token::new(TokenType::Minus, "-".to_string(), None, 0),
                right: Box::new(Expr::Literal(LiteralExpr {
                    value: Some(Object::Num(2.0)),
                })),
            })),
        };
        assert_eq!(
            Object::Num(-35.0),
            interpreter.visit_binary_expr(&expr).unwrap()
        );
    }

    #[test]
    fn t_addition_numbers() {
        let interpreter = Interpreter {};
        let expr = BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Object::Num(70.0)),
            })),
            operator: Token::new(TokenType::Plus, "+".to_string(), None, 0),
            right: Box::new(Expr::Unary(UnaryExpr {
                operator: Token::new(TokenType::Minus, "-".to_string(), None, 0),
                right: Box::new(Expr::Literal(LiteralExpr {
                    value: Some(Object::Num(2.0)),
                })),
            })),
        };
        assert_eq!(
            Object::Num(68.0),
            interpreter.visit_binary_expr(&expr).unwrap()
        );
    }

    #[test]
    fn t_addition_strings() {
        let interpreter = Interpreter {};
        let expr = BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Object::Str("Hello ".to_string())),
            })),
            operator: Token::new(TokenType::Plus, "+".to_string(), None, 0),
            right: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Object::Str("World!".to_string())),
            })),
        };
        assert_eq!(
            Object::Str("Hello World!".to_string()),
            interpreter.visit_binary_expr(&expr).unwrap()
        );
    }
}

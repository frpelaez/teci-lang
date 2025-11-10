use std::cell::RefCell;
use std::rc::Rc;

use crate::envirnoment::Envirnoment;
use crate::error::TeciError;
use crate::expr::*;
use crate::object::Object;
use crate::stmt::{BlockStmt, ExpressionStmt, LetStmt, PrintStmt, Stmt, StmtVisitor};
use crate::token::Token;
use crate::token_type::TokenType;

pub struct Interpreter {
    pub enviroment: RefCell<Rc<RefCell<Envirnoment>>>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            enviroment: RefCell::new(Envirnoment::new()),
        }
    }

    pub fn interpret(&self, statements: &Vec<Stmt>) -> Option<()> {
        for stmt in statements {
            if self.execute(stmt).is_err() {
                return None;
            }
        }
        Some(())
    }

    fn execute(&self, statement: &Stmt) -> Result<(), TeciError> {
        statement.accept(self)
    }

    fn execute_block(
        &self,
        statements: &[Stmt],
        envirnoment: Envirnoment,
    ) -> Result<(), TeciError> {
        println!("curr = {:?}", self.enviroment);
        println!("rep w/ = {:?}", envirnoment);
        let previous = self.enviroment.replace(Rc::new(RefCell::new(envirnoment)));
        println!("prev = {:?}", previous);
        let res = statements
            .iter()
            .try_for_each(|statement| self.execute(statement));
        self.enviroment.replace(previous);
        res
    }

    fn evaluate(&self, expr: &Expr) -> Result<Object, TeciError> {
        expr.accept(self)
    }

    fn is_truthy(obj: &Object) -> bool {
        match obj {
            Object::Num(x) => *x != 0.0,
            Object::Str(s) => !s.is_empty(),
            Object::Bool(b) => *b,
            Object::Nil => false,
            Object::ArithmeticError => false,
            Object::DivisionByZeroError => false,
        }
    }

    fn check_numeric_operands(
        left: &Object,
        right: &Object,
        operator: Token,
    ) -> Result<(), TeciError> {
        match (left, right) {
            (Object::Num(_), Object::Num(_)) => Ok(()),
            _ => Err(TeciError::runtime_error(
                operator,
                "Invalid operator for non numeric operands",
            )),
        }
    }

    pub fn stringify(value: Object) -> String {
        match value {
            Object::Num(x) => {
                let mut text = x.to_string();
                if text.ends_with(".0") {
                    text = text.get(0..text.len() - 2).unwrap().to_string();
                }
                text
            }
            Object::Str(s) => s,
            Object::Bool(b) => b.to_string(),
            Object::Nil => "nil".to_string(),
            Object::ArithmeticError => "arithmetic_error!!!".to_string(),
            Object::DivisionByZeroError => "division_by_zero_error!!!".to_string(),
        }
    }
}

impl ExprVisitor<Object> for Interpreter {
    fn visit_assign_expr(&self, expr: &AssignExpr) -> Result<Object, TeciError> {
        let value = self.evaluate(&expr.value)?;
        self.enviroment
            .borrow()
            .try_borrow_mut()
            .unwrap()
            .assign(&expr.name, value.clone());
        Ok(value)
    }

    fn visit_unary_expr(&self, expr: &UnaryExpr) -> Result<Object, TeciError> {
        let right = self.evaluate(&expr.right)?;
        let result = match expr.operator.ttype {
            TokenType::Minus => -right,
            TokenType::Bang => Object::Bool(!Interpreter::is_truthy(&right)),
            _ => Object::ArithmeticError,
        };

        if result == Object::ArithmeticError {
            Err(TeciError::new(expr.operator.line, "Invalid operator"))
        } else {
            Ok(result)
        }
    }

    fn visit_binary_expr(&self, expr: &BinaryExpr) -> Result<Object, TeciError> {
        let left = self.evaluate(&expr.left)?;
        let right = self.evaluate(&expr.right)?;
        let result = match expr.operator.ttype {
            TokenType::Minus => left - right,
            TokenType::Star => left * right,
            TokenType::Slash => left / right,
            TokenType::Plus => left + right,
            TokenType::GreaterEqual => {
                Interpreter::check_numeric_operands(&left, &right, expr.operator.clone())?;
                Object::Bool(left >= right)
            }
            TokenType::Greater => {
                Interpreter::check_numeric_operands(&left, &right, expr.operator.clone())?;
                Object::Bool(left > right)
            }
            TokenType::LessEqual => {
                Interpreter::check_numeric_operands(&left, &right, expr.operator.clone())?;
                Object::Bool(left <= right)
            }
            TokenType::Less => {
                Interpreter::check_numeric_operands(&left, &right, expr.operator.clone())?;
                Object::Bool(left < right)
            }
            TokenType::Equals => Object::Bool(left == right),
            TokenType::BangEqual => Object::Bool(left != right),
            _ => Object::ArithmeticError,
        };

        if result == Object::ArithmeticError {
            Err(TeciError::runtime_error(
                expr.operator.clone(),
                "Invalid operator",
            ))
        } else if result == Object::DivisionByZeroError {
            Err(TeciError::runtime_error(
                expr.operator.clone(),
                "Division by zero",
            ))
        } else {
            Ok(result)
        }
    }

    fn visit_grouping_expr(&self, expr: &GroupingExpr) -> Result<Object, TeciError> {
        self.evaluate(&expr.expression)
    }

    fn visit_literal_expr(&self, expr: &LiteralExpr) -> Result<Object, TeciError> {
        Ok(expr.value.clone().unwrap())
    }

    fn visit_variable_expr(&self, expr: &VariableExpr) -> Result<Object, TeciError> {
        self.enviroment
            .borrow()
            .try_borrow()
            .unwrap()
            .get(&expr.name)
    }
}

impl StmtVisitor<()> for Interpreter {
    fn visit_print_stmt(&self, stmt: &PrintStmt) -> Result<(), TeciError> {
        let value = self.evaluate(&stmt.expression)?;
        println!("{}", Interpreter::stringify(value));
        Ok(())
    }

    fn visit_expression_stmt(&self, stmt: &ExpressionStmt) -> Result<(), TeciError> {
        self.evaluate(&stmt.expression)?;
        Ok(())
    }

    fn visit_let_stmt(&self, stmt: &LetStmt) -> Result<(), TeciError> {
        let value = if let Some(initializer) = &stmt.initializer {
            self.evaluate(initializer)?
        } else {
            Object::Nil
        };
        self.enviroment
            .borrow()
            .try_borrow_mut()
            .unwrap()
            .define(stmt.name.lexeme.clone(), value);
        Ok(())
    }

    fn visit_block_stmt(&self, stmt: &BlockStmt) -> Result<(), TeciError> {
        self.execute_block(
            &stmt.statements,
            Envirnoment::with_enclosing(self.enviroment.borrow().clone()),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::*;

    #[test]
    fn t_unary_minus() {
        let interpreter = Interpreter::new();
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
        let interpreter = Interpreter::new();
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
        let interpreter = Interpreter::new();
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
        let interpreter = Interpreter::new();
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
        let interpreter = Interpreter::new();
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
        let interpreter = Interpreter::new();
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
    fn t_concatenation_strings() {
        let interpreter = Interpreter::new();
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

    #[test]
    fn t_arithmetic_error() {
        let interpreter = Interpreter::new();
        let expr = BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Object::Str("Hello ".to_string())),
            })),
            operator: Token::new(TokenType::Minus, "-".to_string(), None, 0),
            right: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Object::Str("World!".to_string())),
            })),
        };
        assert!(interpreter.visit_binary_expr(&expr).is_err())
    }

    #[test]
    fn t_greaterequal() {
        let interpreter = Interpreter::new();
        let expr = BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Object::Num(16.0)),
            })),
            operator: Token::new(TokenType::GreaterEqual, ">=".to_string(), None, 0),
            right: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Object::Num(6.0)),
            })),
        };
        assert_eq!(
            Object::Bool(true),
            interpreter.visit_binary_expr(&expr).unwrap()
        )
    }

    #[test]
    fn t_greater() {
        let interpreter = Interpreter::new();
        let expr = BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Object::Num(16.0)),
            })),
            operator: Token::new(TokenType::Greater, ">".to_string(), None, 0),
            right: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Object::Num(6.0)),
            })),
        };
        assert_eq!(
            Object::Bool(true),
            interpreter.visit_binary_expr(&expr).unwrap()
        )
    }

    #[test]
    fn t_lessequal() {
        let interpreter = Interpreter::new();
        let expr = BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Object::Num(6.0)),
            })),
            operator: Token::new(TokenType::LessEqual, "<=".to_string(), None, 0),
            right: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Object::Num(6.0)),
            })),
        };
        assert_eq!(
            Object::Bool(true),
            interpreter.visit_binary_expr(&expr).unwrap()
        )
    }

    #[test]
    fn t_less() {
        let interpreter = Interpreter::new();
        let expr = BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Object::Num(16.0)),
            })),
            operator: Token::new(TokenType::Less, "<".to_string(), None, 0),
            right: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Object::Bool(true)),
            })),
        };
        assert!(interpreter.visit_binary_expr(&expr).is_err())
    }

    #[test]
    fn t_equals() {
        let interpreter = Interpreter::new();
        let expr = BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Object::Str("hello".to_string())),
            })),
            operator: Token::new(TokenType::Equals, "==".to_string(), None, 0),
            right: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Object::Str("hello".to_string())),
            })),
        };
        assert_eq!(
            Object::Bool(true),
            interpreter.visit_binary_expr(&expr).unwrap()
        )
    }

    #[test]
    fn t_bangequals() {
        let interpreter = Interpreter::new();
        let expr = BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Object::Nil),
            })),
            operator: Token::new(TokenType::BangEqual, "!=".to_string(), None, 0),
            right: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Object::Str("sixteen".to_string())),
            })),
        };
        assert_eq!(
            Object::Bool(true),
            interpreter.visit_binary_expr(&expr).unwrap()
        )
    }
}

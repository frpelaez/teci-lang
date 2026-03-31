use std::cell::RefCell;
use std::rc::Rc;

use crate::{
    callable::{Callable, TeciCallable},
    envirnoment::Environment,
    error::TeciResult,
    expr::*,
    native_functions::*,
    object::Object,
    stmt::*,
    teci_function::TeciFunction,
    token::Token,
    token_type::TokenType,
};

pub struct Interpreter {
    pub globals: Rc<RefCell<Environment>>,
    environment: RefCell<Rc<RefCell<Environment>>>,
    nesting_level: RefCell<usize>,
}

impl Interpreter {
    pub fn new() -> Self {
        let globals = Rc::new(RefCell::new(Environment::new()));

        globals.borrow_mut().define(
            "clock",
            Object::Func(Callable {
                func: Rc::new(NativeClock),
            }),
        );

        Self {
            globals: Rc::clone(&globals),
            environment: RefCell::new(Rc::clone(&globals)),
            nesting_level: RefCell::new(0),
        }
    }

    pub fn dbg_environment(&self) {
        println!("{:?}", &self.environment);
    }

    pub fn interpret(&self, statements: &[Stmt]) -> bool {
        *self.nesting_level.borrow_mut() = 0;
        match statements.iter().try_for_each(|s| self.execute(s)) {
            Ok(_) => true,
            Err(r) => matches!(r, TeciResult::Break),
        }
    }

    fn execute(&self, statement: &Stmt) -> Result<(), TeciResult> {
        statement.accept(self)
    }

    pub fn execute_block(
        &self,
        statements: &[Stmt],
        environment: Environment,
    ) -> Result<(), TeciResult> {
        let previous = self.environment.replace(Rc::new(RefCell::new(environment)));
        let result = statements.iter().try_for_each(|s| self.execute(s));
        self.environment.replace(previous);
        result
    }

    fn evaluate(&self, expr: &Expr) -> Result<Object, TeciResult> {
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
            Object::Func(_) => false,
        }
    }

    fn check_numeric_operands(
        left: &Object,
        right: &Object,
        operator: Token,
    ) -> Result<(), TeciResult> {
        match (left, right) {
            (Object::Num(_), Object::Num(_)) => Ok(()),
            _ => Err(TeciResult::runtime_error(
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
            Object::Func(callable) => TeciCallable::to_string(&callable),
        }
    }
}

impl ExprVisitor<Object> for Interpreter {
    fn visit_assign_expr(&self, expr: &AssignExpr) -> Result<Object, TeciResult> {
        let value = self.evaluate(&expr.value)?;
        self.environment
            .borrow()
            .borrow_mut()
            .assign(&expr.name, value.clone())?;
        Ok(value)
    }

    fn visit_unary_expr(&self, expr: &UnaryExpr) -> Result<Object, TeciResult> {
        let right = self.evaluate(&expr.right)?;
        let result = match expr.operator.ttype {
            TokenType::Minus => -right,
            TokenType::Bang => Object::Bool(!Interpreter::is_truthy(&right)),
            _ => Object::ArithmeticError,
        };

        if result == Object::ArithmeticError {
            Err(TeciResult::teci_error(
                expr.operator.line,
                "Invalid operator",
            ))
        } else {
            Ok(result)
        }
    }

    fn visit_binary_expr(&self, expr: &BinaryExpr) -> Result<Object, TeciResult> {
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

        match result {
            Object::ArithmeticError => Err(TeciResult::runtime_error(
                expr.operator.clone(),
                "Invalid operator",
            )),
            Object::DivisionByZeroError => Err(TeciResult::runtime_error(
                expr.operator.clone(),
                "Division by zero",
            )),
            _ => Ok(result),
        }
    }

    fn visit_grouping_expr(&self, expr: &GroupingExpr) -> Result<Object, TeciResult> {
        self.evaluate(&expr.expression)
    }

    fn visit_literal_expr(&self, expr: &LiteralExpr) -> Result<Object, TeciResult> {
        Ok(expr.value.clone().unwrap())
    }

    fn visit_variable_expr(&self, expr: &VariableExpr) -> Result<Object, TeciResult> {
        self.environment.borrow().borrow().get(&expr.name)
    }

    fn visit_logical_expr(&self, expr: &LogicalExpr) -> Result<Object, TeciResult> {
        let left = self.evaluate(&expr.left)?;
        if expr.operator.ttype == TokenType::Or {
            if Interpreter::is_truthy(&left) {
                return Ok(left);
            }
        } else if !Interpreter::is_truthy(&left) {
            return Ok(left);
        }
        self.evaluate(&expr.right)
    }

    fn visit_call_expr(&self, expr: &CallExpr) -> Result<Object, TeciResult> {
        let callee = self.evaluate(&expr.callee)?;

        let mut arguments = Vec::new();
        for arg in &expr.arguments {
            arguments.push(self.evaluate(arg)?);
        }

        if let Object::Func(function) = callee {
            if function.arity() != arguments.len() {
                Err(TeciResult::runtime_error(
                    expr.paren.clone(),
                    &format!(
                        "Expected {} arguments but found {} instead",
                        function.arity(),
                        arguments.len()
                    ),
                ))
            } else {
                function.call(self, arguments)
            }
        } else {
            Err(TeciResult::runtime_error(
                expr.paren.clone(),
                "Only callable objects are functions and classes",
            ))
        }
    }
}

impl StmtVisitor<()> for Interpreter {
    fn visit_print_stmt(&self, stmt: &PrintStmt) -> Result<(), TeciResult> {
        let value = self.evaluate(&stmt.expression)?;
        println!("{}", Interpreter::stringify(value));
        Ok(())
    }

    fn visit_expression_stmt(&self, stmt: &ExpressionStmt) -> Result<(), TeciResult> {
        let _val = self.evaluate(&stmt.expression)?;
        // println!("{}", val);
        Ok(())
    }

    fn visit_let_stmt(&self, stmt: &LetStmt) -> Result<(), TeciResult> {
        let value = if let Some(initializer) = &stmt.initializer {
            self.evaluate(initializer)?
        } else {
            Object::Nil
        };
        self.environment
            .borrow()
            .borrow_mut()
            .define(stmt.name.lexeme.as_str(), value);
        Ok(())
    }

    fn visit_block_stmt(&self, stmt: &BlockStmt) -> Result<(), TeciResult> {
        let e = Environment::with_environment(self.environment.borrow().clone());
        self.execute_block(&stmt.statements, e)
    }

    fn visit_if_stmt(&self, stmt: &IfStmt) -> Result<(), TeciResult> {
        if Interpreter::is_truthy(&self.evaluate(&stmt.condition)?) {
            self.execute(&stmt.then_branch)
        } else if let Some(else_branch) = &stmt.else_branch {
            self.execute(else_branch)
        } else {
            Ok(())
        }
    }

    fn visit_while_stmt(&self, stmt: &WhileStmt) -> Result<(), TeciResult> {
        *self.nesting_level.borrow_mut() += 1;
        while Interpreter::is_truthy(&self.evaluate(&stmt.condition)?) {
            match self.execute(&stmt.body) {
                Ok(_) => {}
                Err(e) => match e {
                    TeciResult::Break => {
                        break;
                    }
                    _ => return Err(e),
                },
            }
        }

        *self.nesting_level.borrow_mut() -= 1;

        Ok(())
    }

    fn visit_break_stmt(&self, stmt: &BreakStmt) -> Result<(), TeciResult> {
        if *self.nesting_level.borrow() == 0 {
            Err(TeciResult::runtime_error(
                stmt.token.clone(),
                "Found a 'break' statement outside a loop",
            ))
        } else {
            Err(TeciResult::Break)
        }
    }

    fn visit_function_stmt(&self, stmt: &FunctionStmt) -> Result<(), TeciResult> {
        let function = TeciFunction::new(stmt.clone());
        self.environment.borrow().borrow_mut().define(
            &stmt.name.lexeme,
            Object::Func(Callable {
                func: Rc::new(function),
            }),
        );
        Ok(())
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

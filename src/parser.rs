use crate::{
    error::TeciError,
    expr::{AssignExpr, BinaryExpr, Expr, GroupingExpr, LiteralExpr, UnaryExpr, VariableExpr},
    object::Object,
    stmt::{ExpressionStmt, LetStmt, PrintStmt, Stmt},
    token::Token,
    token_type::TokenType,
};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    had_error: bool,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0,
            had_error: false,
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, TeciError> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            statements.push(self.declaration()?);
        }
        Ok(statements)
    }

    pub fn succeded(&self) -> bool {
        !self.had_error
    }

    fn declaration(&mut self) -> Result<Stmt, TeciError> {
        let res = if self.is_match(&[TokenType::Let]) {
            self.let_declaration()
        } else {
            self.statement()
        };
        if res.is_err() {
            self.synchronize();
        };
        res
    }

    fn let_declaration(&mut self) -> Result<Stmt, TeciError> {
        let name = self.consume(TokenType::Identifier, "Expected variable name")?;
        let initializer = if self.is_match(&[TokenType::Assign]) {
            Some(self.expression()?)
        } else {
            None
        };
        self.consume(
            TokenType::Semicolon,
            "Expected ';' after variable declaration",
        )?;
        Ok(Stmt::Let(LetStmt { name, initializer }))
    }

    fn statement(&mut self) -> Result<Stmt, TeciError> {
        if self.is_match(&[TokenType::Print]) {
            self.print_statement()
        } else {
            self.expr_statement()
        }
    }

    fn print_statement(&mut self) -> Result<Stmt, TeciError> {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, "Expected ';' after expression")?;
        Ok(Stmt::Print(PrintStmt { expression: expr }))
    }

    fn expr_statement(&mut self) -> Result<Stmt, TeciError> {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, "Expected ';' after expression")?;
        Ok(Stmt::Expression(ExpressionStmt { expression: expr }))
    }

    fn expression(&mut self) -> Result<Expr, TeciError> {
        self.assignment()
        /* TODO: comma expressions -> create new kind of expression and parse it

        let mut expr = self.equality()?;
        while self.is_match(&[TokenType::Comma]) {
            expr = self.equality()?;
        }
        Ok(expr) */
    }

    fn assignment(&mut self) -> Result<Expr, TeciError> {
        let expr = self.equality()?;
        if self.is_match(&[TokenType::Assign]) {
            let equals = self.previous();
            let value = self.assignment()?;
            if let Expr::Variable(var_exp) = expr {
                let name = var_exp.name;
                return Ok(Expr::Assign(AssignExpr {
                    name,
                    value: Box::new(value),
                }));
            }
            self.error(equals, "Invalid assignment target");
        }
        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expr, TeciError> {
        let mut expr = self.comparison()?;
        while self.is_match(&[TokenType::BangEqual, TokenType::Equals]) {
            let operator = self.previous();
            let right = self.comparison()?;
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }
        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, TeciError> {
        let mut expr = self.term()?;
        while self.is_match(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous();
            let right = self.term()?;
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }
        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, TeciError> {
        let mut expr = self.factor()?;
        while self.is_match(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous();
            let right = self.factor()?;
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }
        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, TeciError> {
        let mut expr = self.unary()?;
        while self.is_match(&[TokenType::Star, TokenType::Slash]) {
            let operator = self.previous();
            let right = self.unary()?;
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }
        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, TeciError> {
        if self.is_match(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let right = self.unary()?;
            Ok(Expr::Unary(UnaryExpr {
                operator,
                right: Box::new(right),
            }))
        } else {
            Ok(self.primary()?)
        }
    }

    fn primary(&mut self) -> Result<Expr, TeciError> {
        if self.is_match(&[TokenType::False]) {
            return Ok(Expr::Literal(LiteralExpr {
                value: Some(Object::Bool(false)),
            }));
        }
        if self.is_match(&[TokenType::True]) {
            return Ok(Expr::Literal(LiteralExpr {
                value: Some(Object::Bool(true)),
            }));
        }
        if self.is_match(&[TokenType::Nil]) {
            return Ok(Expr::Literal(LiteralExpr {
                value: Some(Object::Nil),
            }));
        }
        if self.is_match(&[TokenType::String, TokenType::Number]) {
            return Ok(Expr::Literal(LiteralExpr {
                value: self.previous().literal,
            }));
        }
        if self.is_match(&[TokenType::LeftParen]) {
            let expr = self.expression()?;
            self.consume(TokenType::RightParen, "Expected ')' after expression")?;
            return Ok(Expr::Grouping(GroupingExpr {
                expression: Box::new(expr),
            }));
        }
        if self.is_match(&[TokenType::Identifier]) {
            return Ok(Expr::Variable(VariableExpr {
                name: self.previous(),
            }));
        }

        Err(TeciError::new(0, "unimplemented (yet)"))
    }

    fn consume(&mut self, ttype: TokenType, error_message: &str) -> Result<Token, TeciError> {
        if self.check(ttype) {
            Ok(self.advance())
        } else {
            Err(self.error(self.peek(), error_message))
        }
    }

    fn error(&mut self, token: Token, message: &str) -> TeciError {
        self.had_error = true;
        TeciError::parse_error(token, message)
    }

    fn synchronize(&mut self) {
        self.advance();
        while !self.is_at_end() {
            if self.previous().ttype == TokenType::Semicolon {
                return;
            }
            if matches!(
                self.peek().ttype,
                TokenType::Class
                    | TokenType::Fun
                    | TokenType::Let
                    | TokenType::For
                    | TokenType::If
                    | TokenType::While
                    | TokenType::Print
                    | TokenType::Return
            ) {
                return;
            }
            self.advance();
        }
    }

    fn is_match(&mut self, types: &[TokenType]) -> bool {
        for ttype in types {
            if self.check(*ttype) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, ttype: TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            self.peek().ttype == ttype
        }
    }

    fn peek(&self) -> Token {
        self.tokens.get(self.current).unwrap().clone()
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn previous(&self) -> Token {
        self.tokens.get(self.current - 1).unwrap().clone()
    }

    fn is_at_end(&self) -> bool {
        self.peek().ttype == TokenType::Eof
    }
}

use crate::{
    error::TeciResult,
    expr::{
        AssignExpr, BinaryExpr, Expr, GroupingExpr, LiteralExpr, LogicalExpr, UnaryExpr,
        VariableExpr,
    },
    object::Object,
    stmt::{BlockStmt, BreakStmt, ExpressionStmt, IfStmt, LetStmt, PrintStmt, Stmt, WhileStmt},
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

    pub fn parse(&mut self) -> Result<Vec<Stmt>, TeciResult> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            statements.push(self.declaration()?);
        }

        Ok(statements)
    }

    pub fn succeded(&self) -> bool {
        !self.had_error
    }

    fn declaration(&mut self) -> Result<Stmt, TeciResult> {
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

    fn let_declaration(&mut self) -> Result<Stmt, TeciResult> {
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

    fn statement(&mut self) -> Result<Stmt, TeciResult> {
        if self.is_match(&[TokenType::Break]) {
            self.break_statement()
        } else if self.is_match(&[TokenType::If]) {
            self.if_statement()
        } else if self.is_match(&[TokenType::Print]) {
            self.print_statement()
        } else if self.is_match(&[TokenType::While]) {
            self.while_statement()
        } else if self.is_match(&[TokenType::For]) {
            self.for_statement()
        } else if self.is_match(&[TokenType::LeftBrace]) {
            // I do this in order to be able to reuse the self.block() for other block parsing in the future
            Ok(Stmt::Block(BlockStmt {
                statements: self.block()?,
            }))
        } else {
            self.expr_statement()
        }
    }

    fn print_statement(&mut self) -> Result<Stmt, TeciResult> {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, "Expected ';' after expression")?;

        Ok(Stmt::Print(PrintStmt { expression: expr }))
    }

    fn expr_statement(&mut self) -> Result<Stmt, TeciResult> {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, "Expected ';' after expression")?;

        Ok(Stmt::Expression(ExpressionStmt { expression: expr }))
    }

    fn block(&mut self) -> Result<Vec<Stmt>, TeciResult> {
        let mut statements = Vec::new();
        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            statements.push(self.declaration()?);
        }
        self.consume(TokenType::RightBrace, "Expected '}' after declarations")?;

        Ok(statements)
    }

    fn if_statement(&mut self) -> Result<Stmt, TeciResult> {
        self.consume(TokenType::LeftParen, "Expected '(' after 'if'")?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "Expected ')' after if condition")?;

        let then_branch = Box::new(self.statement()?);
        let else_branch = if self.is_match(&[TokenType::Else]) {
            Some(Box::new(self.statement()?))
        } else {
            None
        };

        Ok(Stmt::If(IfStmt {
            condition,
            then_branch,
            else_branch,
        }))
    }

    fn while_statement(&mut self) -> Result<Stmt, TeciResult> {
        self.consume(TokenType::LeftParen, "Expected '(' after 'while'")?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "Expected ')' after while condition")?;
        let body = Box::new(self.statement()?);

        Ok(Stmt::While(WhileStmt { condition, body }))
    }

    fn for_statement(&mut self) -> Result<Stmt, TeciResult> {
        self.consume(TokenType::LeftParen, "Expected '(' after 'for'")?;

        let initializer = if self.is_match(&[TokenType::Semicolon]) {
            None
        } else if self.is_match(&[TokenType::Let]) {
            Some(self.let_declaration()?)
        } else {
            Some(self.expr_statement()?)
        };

        let condition = if self.check(TokenType::Semicolon) {
            None
        } else {
            Some(self.expression()?)
        };
        self.consume(TokenType::Semicolon, "Expected ';' after loop condition")?;

        let increment = if self.check(TokenType::RightParen) {
            None
        } else {
            Some(self.expression()?)
        };
        self.consume(TokenType::RightParen, "Expected ')' after for clauses")?;

        let mut body = self.statement()?;

        if let Some(inc) = increment {
            body = Stmt::Block(BlockStmt {
                statements: vec![body, Stmt::Expression(ExpressionStmt { expression: inc })],
            });
        }

        body = Stmt::While(WhileStmt {
            condition: if let Some(cond) = condition {
                cond
            } else {
                Expr::Literal(LiteralExpr {
                    value: Some(Object::Bool(true)),
                })
            },
            body: Box::new(body),
        });

        if let Some(init) = initializer {
            body = Stmt::Block(BlockStmt {
                statements: vec![init, body],
            });
        }

        Ok(body)
    }

    fn break_statement(&mut self) -> Result<Stmt, TeciResult> {
        self.consume(TokenType::Semicolon, "Expected ';' after 'break'")?;
        Ok(Stmt::Break(BreakStmt { _a: Some(()) }))
    }

    fn expression(&mut self) -> Result<Expr, TeciResult> {
        self.assignment()
        /* TODO: comma expressions -> create new kind of expression and parse it
        let mut expr = self.equality()?;
        while self.is_match(&[TokenType::Comma]) {
            expr = self.equality()?;
        }
        Ok(expr) */
    }

    fn assignment(&mut self) -> Result<Expr, TeciResult> {
        let expr = self.or()?;

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

    fn or(&mut self) -> Result<Expr, TeciResult> {
        let mut expr = self.and()?;
        while self.is_match(&[TokenType::Or]) {
            let operator = self.previous();
            let right = self.and()?;
            expr = Expr::Logical(LogicalExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        Ok(expr)
    }

    fn and(&mut self) -> Result<Expr, TeciResult> {
        let mut expr = self.equality()?;
        while self.is_match(&[TokenType::And]) {
            let operator = self.previous();
            let right = self.and()?;
            expr = Expr::Logical(LogicalExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expr, TeciResult> {
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

    fn comparison(&mut self) -> Result<Expr, TeciResult> {
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

    fn term(&mut self) -> Result<Expr, TeciResult> {
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

    fn factor(&mut self) -> Result<Expr, TeciResult> {
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

    fn unary(&mut self) -> Result<Expr, TeciResult> {
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

    fn primary(&mut self) -> Result<Expr, TeciResult> {
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

        Err(TeciResult::teci_error(0, "unimplemented (yet)"))
    }

    fn consume(&mut self, ttype: TokenType, error_message: &str) -> Result<Token, TeciResult> {
        if self.check(ttype) {
            Ok(self.advance())
        } else {
            Err(self.error(self.peek(), error_message))
        }
    }

    fn error(&mut self, token: Token, message: &str) -> TeciResult {
        self.had_error = true;
        TeciResult::parse_error(token, message)
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

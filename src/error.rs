use crate::{token::Token, token_type::TokenType};

#[derive(Debug)]
pub struct TeciError {
    token: Option<Token>,
    line: usize,
    message: String,
}

impl TeciError {
    pub fn new(line: usize, message: &str) -> Self {
        let error = TeciError {
            token: None,
            line,
            message: message.to_string(),
        };
        error.report("");
        error
    }

    pub fn parse_error(token: Token, message: &str) -> TeciError {
        let line = token.line;
        let error = TeciError {
            token: Some(token),
            line,
            message: message.to_string(),
        };
        error.report("");
        error
    }

    pub fn runtime_error(token: Token, message: &str) -> TeciError {
        let line = token.line;
        let error = TeciError {
            token: Some(token),
            line,
            message: message.to_string(),
        };
        error.report("");
        error
    }

    pub fn report(&self, loc: &str) {
        if let Some(token) = self.token.clone() {
            if token.ttype == TokenType::Eof {
                eprintln!("[line {}] Error: {} at end", self.line, self.message);
            } else {
                eprintln!(
                    "[line {}] Error: {} at '{}'",
                    self.line, self.message, token.lexeme
                )
            }
        } else {
            eprintln!("[line {}] Error: {} {}", self.line, loc, self.message);
        }
    }
}

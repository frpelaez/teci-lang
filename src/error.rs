use crate::{token::Token, token_type::TokenType};

#[derive(Debug)]
pub struct TeciError {
    token: Option<Token>,
    line: usize,
    message: String,
}

impl TeciError {
    pub fn new(line: usize, message: String) -> Self {
        let error = TeciError {
            token: None,
            line,
            message,
        };
        error.report("".to_string());
        error
    }

    pub fn parse_error(token: Token, message: String) -> TeciError {
        let line = token.line;
        let error = TeciError {
            token: Some(token),
            line,
            message,
        };
        error.report("".to_string());
        error
    }

    pub fn report(&self, loc: String) {
        if let Some(token) = self.token.clone() {
            if token.ttype == TokenType::Eof {
                eprintln!("[line {}] Error: at end {}", self.line, self.message);
            } else {
                eprintln!(
                    "[line {}] Error: {} at {}",
                    self.line, token.lexeme, self.message
                )
            }
        } else {
            eprintln!("[line {}] Error: {} {}", self.line, loc, self.message);
        }
    }
}

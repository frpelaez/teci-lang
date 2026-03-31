use crate::{token::Token, token_type::TokenType};

#[derive(Debug)]
pub enum TeciResult {
    ParseError { token: Token, message: String },
    RuntimeError { token: Token, message: String },
    TeciError { line: usize, message: String },
    Break,
}

impl TeciResult {
    pub fn parse_error(token: Token, message: &str) -> TeciResult {
        let error = TeciResult::ParseError {
            token,
            message: message.to_string(),
        };
        error.report("");
        error
    }

    pub fn runtime_error(token: Token, message: &str) -> TeciResult {
        let error = TeciResult::RuntimeError {
            token,
            message: message.to_string(),
        };
        error.report("");
        error
    }

    pub fn teci_error(line: usize, message: &str) -> TeciResult {
        let error = TeciResult::TeciError {
            line,
            message: message.to_string(),
        };
        error.report("");
        error
    }

    pub fn report(&self, loc: &str) {
        match self {
            TeciResult::ParseError { token, message } => {
                let token_display = match &token.ttype {
                    TokenType::Eof => "EOF",
                    _ => &format!("{:?}::{}", token.ttype, token.lexeme),
                };
                eprintln!(
                    "[Parse Error] In line {} at '{}': {}",
                    token.line, token_display, message
                )
            }
            TeciResult::RuntimeError { token, message } => {
                let token_display = match &token.ttype {
                    TokenType::Eof => "EOF",
                    _ => &format!("{:?}::{}", token.ttype, token.lexeme),
                };
                eprintln!(
                    "[Runtime Error] In line {} at '{}': {}",
                    token.line, token_display, message
                )
            }
            TeciResult::TeciError { line, message } => {
                eprintln!("[Error {}] In line {} : {}", loc, line, message)
            }
            TeciResult::Break => {}
        }
    }
}

use crate::{
    error::TeciError,
    token::{Object, Token},
    token_type::TokenType,
};

#[allow(dead_code)]
#[derive(Debug)]
pub struct Scanner {
    source: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Scanner {
            source: source.chars().collect(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Result<&Vec<Token>, TeciError> {
        let mut had_error: Option<TeciError> = None;
        while !self.is_at_end() {
            self.start = self.current;
            if let Err(e) = self.scan_token() {
                e.report("".to_string());
                had_error = Some(e);
            }
        }

        self.tokens
            .push(Token::new(TokenType::Eof, "".to_string(), None, self.line));

        if let Some(e) = had_error {
            Err(e)
        } else {
            Ok(&self.tokens)
        }
    }

    fn scan_token(&mut self) -> Result<(), TeciError> {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            ';' => self.add_token(TokenType::Semicolon),
            '+' => self.add_token(TokenType::Plus),
            '-' => self.add_token(TokenType::Minus),
            '*' => self.add_token(TokenType::Star),
            '!' => {
                let ttype = if self.next_is_and_advance('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                self.add_token(ttype);
            }
            '<' => {
                let ttype = if self.next_is_and_advance('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.add_token(ttype);
            }
            '>' => {
                let ttype = if self.next_is_and_advance('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                self.add_token(ttype);
            }
            '=' => {
                let ttype = if self.next_is_and_advance('=') {
                    TokenType::Equals
                } else {
                    TokenType::Assign
                };
                self.add_token(ttype);
            }
            _ => {
                return Err(TeciError::new(
                    self.line,
                    "Unexpected character.".to_string(),
                ));
            }
        }
        Ok(())
    }

    fn add_token(&mut self, ttype: TokenType) {
        self.add_token_object(ttype, None);
    }

    fn add_token_object(&mut self, ttype: TokenType, literal: Option<Object>) {
        let lexeme = self
            .source
            .get(self.start..self.current)
            .unwrap()
            .iter()
            .collect();
        self.tokens
            .push(Token::new(ttype, lexeme, literal, self.line));
    }

    fn advance(&mut self) -> char {
        let next = *self.source.get(self.current).unwrap();
        self.current += 1;
        next
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn next_is_and_advance(&mut self, expected: char) -> bool {
        if self.is_at_end() || *self.source.get(self.current).unwrap() != expected {
            return false;
        }
        self.current += 1;
        true
    }
}

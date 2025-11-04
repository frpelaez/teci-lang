use crate::{error::TeciError, object::Object, token::Token, token_type::TokenType};

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

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, TeciError> {
        let mut had_error: Option<TeciError> = None;
        while !self.is_at_end() {
            self.start = self.current;
            if let Err(e) = self.scan_token() {
                e.report("");
                had_error = Some(e);
            }
        }

        self.tokens
            .push(Token::new(TokenType::Eof, "".to_string(), None, self.line));

        if let Some(e) = had_error {
            Err(e)
        } else {
            Ok(self.tokens.clone())
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
            '/' => {
                if self.next_is_and_advance('/') {
                    // Coments go with '//' and reach until the end of the line
                    while let Some(ch) = self.peek() {
                        if ch != '\n' {
                            self.advance();
                        } else {
                            break;
                        }
                    }
                } else if self.next_is_and_advance('*') {
                    // Block comment starts
                    self.read_comment()?;
                } else {
                    self.add_token(TokenType::Slash);
                }
            }
            ' ' | '\r' | '\t' => {}
            '\n' => {
                self.line += 1;
            }
            '"' => self.read_string()?,
            '0'..='9' => self.read_number(),
            _ => {
                if c.is_ascii_alphabetic() || c == '_' {
                    self.read_identifier()?;
                }
            } // _ => {
              //     return Err(TeciError::new(
              //         self.line,
              //         "Unexpected character.".to_string(),
              //     ));
              // }
        }
        Ok(())
    }

    fn read_identifier(&mut self) -> Result<(), TeciError> {
        while Scanner::is_alphanumeric(self.peek()) {
            self.advance();
        }

        let check: String = self
            .source
            .get(self.start..self.current)
            .unwrap()
            .iter()
            .collect();
        if let Some(ttype) = Scanner::keyword(check.as_str()) {
            self.add_token(ttype);
        } else {
            self.add_token(TokenType::Identifier);
        }

        Ok(())
    }

    fn read_string(&mut self) -> Result<(), TeciError> {
        while let Some(ch) = self.peek() {
            match ch {
                '"' => break,
                '\n' => {
                    self.line += 1;
                }
                _ => {}
            }
            self.advance();
        }

        if self.is_at_end() {
            return Err(TeciError::new(self.line, "Unterminated string."));
        }

        self.advance();

        // TODO: allow escape sequence, e.g. "\n"

        let literal = self
            .source
            .get(self.start + 1..self.current - 1)
            .unwrap()
            .iter()
            .collect();
        self.add_token_object(TokenType::String, Some(Object::Str(literal)));

        Ok(())
    }

    fn read_comment(&mut self) -> Result<(), TeciError> {
        loop {
            match self.peek() {
                Some('*') => {
                    self.advance();
                    if self.next_is_and_advance('/') {
                        return Ok(());
                    }
                }
                Some('/') => {
                    self.advance();
                    if self.next_is_and_advance('*') {
                        self.read_comment()?;
                    }
                }
                Some('\n') => {
                    self.advance();
                    self.line += 1;
                }
                None => {
                    return Err(TeciError::new(self.line, "Unterminated comment."));
                }
                _ => {
                    self.advance();
                }
            }
        }
    }

    fn read_number(&mut self) {
        while Scanner::is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == Some('.') && Scanner::is_digit(self.peek_next()) {
            self.advance();
            while Scanner::is_digit(self.peek()) {
                self.advance();
            }
        }

        let value: String = self
            .source
            .get(self.start..self.current)
            .unwrap()
            .iter()
            .collect();
        self.add_token_object(TokenType::Number, Some(Object::Num(value.parse().unwrap())));
    }

    fn is_digit(ch: Option<char>) -> bool {
        if let Some(ch) = ch {
            ch.is_ascii_digit()
        } else {
            false
        }
    }

    fn is_alphanumeric(ch: Option<char>) -> bool {
        if let Some(ch) = ch {
            ch.is_ascii_alphanumeric() || ch == '_'
        } else {
            false
        }
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
        match self.source.get(self.current) {
            Some(ch) if *ch == expected => {
                self.current += 1;
                true
            }
            _ => false,
        }
    }

    fn peek(&self) -> Option<char> {
        self.source.get(self.current).copied()
    }

    fn peek_next(&self) -> Option<char> {
        self.source.get(self.current + 1).copied()
    }

    fn keyword(check: &str) -> Option<TokenType> {
        match check {
            "and" => Some(TokenType::And),
            "class" => Some(TokenType::Class),
            "else" => Some(TokenType::Else),
            "false" => Some(TokenType::False),
            "for" => Some(TokenType::For),
            "fun" => Some(TokenType::Fun),
            "if" => Some(TokenType::If),
            "let" => Some(TokenType::Let),
            "nil" => Some(TokenType::Nil),
            "or" => Some(TokenType::Or),
            "print" => Some(TokenType::Print),
            "return" => Some(TokenType::Return),
            "super" => Some(TokenType::Super),
            "this" => Some(TokenType::This),
            "true" => Some(TokenType::True),
            "while" => Some(TokenType::While),
            _ => None,
        }
    }
}

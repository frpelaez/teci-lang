#[allow(dead_code)]
#[derive(Debug)]
pub enum TokenType {
    // One char tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Plus,
    Minus,
    Star,
    Slash,
    Semicolon,
    // One or two char tokens
    Bang,
    BangEqual,
    Assign,
    Equals,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    // Literals
    Number,
    Identifier,
    String,
    // Keywords
    And,
    Or,
    If,
    Else,
    True,
    False,
    For,
    While,
    Fun,
    Class,
    Return,
    Let,
    This,
    Super,
    Print,
    Nil,
    // EOF
    Eof,
}

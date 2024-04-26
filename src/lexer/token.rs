use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum Token {
    ILLEGAL,
    EOF,

    // Identifiers and literals
    IDENT(String),
    INT(String),

    // Operators
    ASSIGN,
    PLUS,
    MINUS,
    EQUAL,
    NOTEQUAL,
    LESSTHAN,
    GREATERTHAN,
    BANG,
    SLASH,
    ASTERISK,

    // Delimiters
    COMMA,
    SEMICOLON,
    LPAREN,
    RPAREN,
    LSQUIRLY,
    RSQUIRLY,

    // Keywords
    FUNCTION,
    LET,
    IF,
    RETURN,
    TRUE,
    FALSE,
    ELSE,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::IDENT(value) | Token::INT(value) => write!(f, "{value}"),
            _ => write!(f, "{:?}", self),
        }
    }
}

impl From<u8> for Token {
    fn from(ch: u8) -> Self {
        match ch {
            b'=' => Token::ASSIGN,
            b'+' => Token::PLUS,
            b'-' => Token::MINUS,
            b'<' => Token::LESSTHAN,
            b'>' => Token::GREATERTHAN,
            b'!' => Token::BANG,
            b'/' => Token::SLASH,
            b'*' => Token::ASTERISK,
            b',' => Token::COMMA,
            b';' => Token::SEMICOLON,
            b'(' => Token::LPAREN,
            b')' => Token::RPAREN,
            b'{' => Token::LSQUIRLY,
            b'}' => Token::RSQUIRLY,
            0 => Token::EOF,
            _ => Token::ILLEGAL,
        }
    }
}

impl From<String> for Token {
    fn from(value: String) -> Self {
        match value.as_str() {
            "==" => Self::EQUAL,
            "!=" => Self::NOTEQUAL,
            "fn" => Self::FUNCTION,
            "let" => Self::LET,
            "true" => Self::TRUE,
            "false" => Self::FALSE,
            "if" => Self::IF,
            "else" => Self::ELSE,
            "return" => Self::RETURN,
            _ => Self::IDENT(value),
        }
    }
}

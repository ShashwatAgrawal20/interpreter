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

    // Delimiters
    COMMA,
    SEMICOLON,

    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

    // Keywords
    FUNCTION,
    LET,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::ILLEGAL => write!(f, "ILLEGAL"),
            Token::EOF => write!(f, "EOF"),
            Token::IDENT(value) => write!(f, "IDENT({value})"),
            Token::INT(value) => write!(f, "INT({value})"),
            Token::ASSIGN => write!(f, "ASSIGN"),
            Token::PLUS => write!(f, "PLUS"),
            Token::COMMA => write!(f, "COMMA"),
            Token::SEMICOLON => write!(f, "SEMICOLON"),
            Token::LPAREN => write!(f, "LPAREN"),
            Token::RPAREN => write!(f, "RPAREN"),
            Token::LBRACE => write!(f, "LBRACE"),
            Token::RBRACE => write!(f, "RBRACE"),
            Token::FUNCTION => write!(f, "FUNCTION"),
            Token::LET => write!(f, "LET"),
        }
    }
}

impl From<u8> for Token {
    fn from(ch: u8) -> Self {
        match ch {
            b'=' => Token::ASSIGN,
            b'+' => Token::PLUS,
            b',' => Token::COMMA,
            b';' => Token::SEMICOLON,
            b'(' => Token::LPAREN,
            b')' => Token::RPAREN,
            b'{' => Token::LBRACE,
            b'}' => Token::RBRACE,
            0 => Token::EOF,
            _ => Token::ILLEGAL,
        }
    }
}

impl From<String> for Token {
    fn from(value: String) -> Self {
        match value.as_str() {
            "fn" => Token::FUNCTION,
            "let" => Token::LET,
            _ => {
                if value.chars().all(|b| b.is_ascii_digit()) {
                    Self::INT(value)
                } else {
                    Self::IDENT(value)
                }
            }
        }
    }
}

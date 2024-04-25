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
            Token::ILLEGAL => write!(f, "ILLEGAL"),
            Token::EOF => write!(f, "EOF"),
            Token::IDENT(value) | Token::INT(value) => write!(f, "{value}"),
            Token::ASSIGN => write!(f, "ASSIGN"),
            Token::PLUS => write!(f, "PLUS"),
            Token::MINUS => write!(f, "MINUS"),
            Token::EQUAL => write!(f, "EQUAL"),
            Token::NOTEQUAL => write!(f, "NOTEQUAL"),
            Token::LESSTHAN => write!(f, "LESSTHAN"),
            Token::GREATERTHAN => write!(f, "GREATERTHAN"),
            Token::BANG => write!(f, "BANG"),
            Token::SLASH => write!(f, "SLASH"),
            Token::ASTERISK => write!(f, "ASTERISK"),
            Token::COMMA => write!(f, "COMMA"),
            Token::SEMICOLON => write!(f, "SEMICOLON"),
            Token::LPAREN => write!(f, "LPAREN"),
            Token::RPAREN => write!(f, "RPAREN"),
            Token::LSQUIRLY => write!(f, "LSQUIRLY"),
            Token::RSQUIRLY => write!(f, "RSQUIRLY"),
            Token::FUNCTION => write!(f, "FUNCTION"),
            Token::LET => write!(f, "LET"),
            Token::IF => write!(f, "IF"),
            Token::RETURN => write!(f, "RETURN"),
            Token::TRUE => write!(f, "TRUE"),
            Token::FALSE => write!(f, "FALSE"),
            Token::ELSE => write!(f, "ELSE"),
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

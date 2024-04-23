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

impl From<char> for Token {
    fn from(ch: char) -> Self {
        match ch {
            '=' => Token::ASSIGN,
            '+' => Token::PLUS,
            ',' => Token::COMMA,
            ';' => Token::SEMICOLON,
            '(' => Token::LPAREN,
            ')' => Token::RPAREN,
            '{' => Token::LBRACE,
            '}' => Token::RBRACE,
            '\0' => Token::EOF,
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn shitty_single_token() {
        // strings
        assert_eq!(Token::from("fn".to_string()), Token::FUNCTION);
        assert_eq!(Token::from("let".to_string()), Token::LET);
        // integers and identifiers
        assert_eq!(Token::from("1".to_string()), Token::INT("1".to_string()));
        assert_eq!(
            Token::from("my_var".to_string()),
            Token::IDENT("my_var".to_string())
        );
        // characters
        assert_eq!(Token::from('='), Token::ASSIGN);
        assert_eq!(Token::from('+'), Token::PLUS);
        assert_eq!(Token::from(','), Token::COMMA);
        assert_eq!(Token::from(';'), Token::SEMICOLON);
        assert_eq!(Token::from('('), Token::LPAREN);
        assert_eq!(Token::from(')'), Token::RPAREN);
        assert_eq!(Token::from('{'), Token::LBRACE);
        assert_eq!(Token::from('}'), Token::RBRACE);
        assert_eq!(Token::from('\0'), Token::EOF);
        assert_eq!(Token::from(' '), Token::ILLEGAL);
    }
}

mod testies;
mod token;
pub use token::Token;

pub struct Lexer {
    // going full on u8 nuts, just don't wanna deal with those shitty utf8.
    // perf this shit
    input: Vec<u8>,
    position: usize,
    read_position: usize,
    ch: u8,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut lexer = Lexer {
            input: input.into_bytes(),
            position: 0,
            read_position: 0,
            ch: 0,
        };
        lexer.read_char();
        lexer
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let token = match self.ch {
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => return Token::from(self.read_identifier()),
            b'0'..=b'9' => return Token::INT(self.read_int()),
            b'=' | b'!' => {
                if self.peek() == b'=' {
                    let ch = self.ch as char;
                    self.read_char();
                    Token::from(format!("{ch}{}", self.ch as char))
                } else {
                    Token::from(self.ch)
                }
            }
            _ => Token::from(self.ch),
        };
        self.read_char();
        token
    }

    fn peek(&self) -> u8 {
        if self.read_position >= self.input.len() {
            return 0;
        } else {
            return self.input[self.read_position];
        }
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read_char();
        }
    }

    fn read_identifier(&mut self) -> String {
        let pos = self.position;
        while self.ch.is_ascii_alphabetic() || self.ch == b'_' {
            self.read_char();
        }
        return String::from_utf8_lossy(&self.input[pos..self.position]).to_string();
    }

    fn read_int(&mut self) -> String {
        let pos = self.position;
        while self.ch.is_ascii_digit() {
            self.read_char();
        }
        return String::from_utf8_lossy(&self.input[pos..self.position]).to_string();
    }
}

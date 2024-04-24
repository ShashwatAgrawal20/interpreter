use super::Token;

pub struct Lexer {
    // going full on u8 nuts, just don't wanna deal with those shitty utf8.
    // perf this shit
    input: Vec<u8>,
    position: usize,
    read_position: usize,
    ch: u8,
}

impl Lexer {
    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1
    }

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
        let token = Token::from(self.ch);
        self.read_char();
        token
    }
}

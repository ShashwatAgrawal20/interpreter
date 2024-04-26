use crate::lexer::{Lexer, Token};
use std::io::Write;

pub fn main() {
    println!("A bullshit interpreter(jk it's just a lexer)");
    loop {
        print!("> ");
        std::io::stdout().flush().expect("Error flushing stdout");
        let mut input = String::new();

        if let Err(e) = std::io::stdin().read_line(&mut input) {
            eprintln!("Error reading input: {}", e);
            continue;
        }

        let mut lexer = Lexer::new(input);
        loop {
            let token = lexer.next_token();
            match token {
                Token::EOF => break,
                _ => println!("{:?}", token),
            }
        }
    }
}

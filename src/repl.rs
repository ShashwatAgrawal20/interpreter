use crate::{
    // lexer::{Lexer, Token},
    lexer::Lexer,
    parser::Parser,
};
use std::io::Write;

pub fn main() {
    println!("possibly the worst code you might ever see");
    loop {
        print!("> ");
        std::io::stdout().flush().expect("Error flushing stdout");
        let mut input = String::new();

        if let Err(e) = std::io::stdin().read_line(&mut input) {
            eprintln!("Error reading input: {}", e);
            continue;
        }

        let lexer = Lexer::new(input.into());
        // ----------------- Parser ------------------
        // loop {
        //     let token = lexer.next_token();
        //     match token {
        //         Token::EOF => break,
        //         _ => println!("{:?}", token),
        //     }
        // }
        // ----------------- Parser ------------------

        // -----------------   AST  ------------------
        let mut parser = Parser::new(lexer);
        let program_ast = parser.parse_program();
        if parser.errors.len() > 0 {
            println!("{:?}", parser.errors);
        } else {
            println!("{:#?}", program_ast);
        }
        // -----------------   AST  ------------------
    }
}

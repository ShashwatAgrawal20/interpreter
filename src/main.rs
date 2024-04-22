pub mod lexer;

fn main() {
    let ex_token = lexer::Token::IDENT(String::from("20"));
    println!("{}", ex_token);
}

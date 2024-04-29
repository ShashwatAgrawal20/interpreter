#[test]
fn parser_creation() {
    use super::{Lexer, Parser};

    let input = "let bullshit = 69";
    let lexer = Lexer::new(input.into());
    let parser = Parser::new(lexer);
    assert_eq!(parser.cur_token, super::Token::LET);
    assert_eq!(
        parser.peek_token,
        super::Token::IDENT("bullshit".to_string())
    );
}

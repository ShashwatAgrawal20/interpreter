#[test]
fn individual_token_test() {
    use super::Token;
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
    assert_eq!(Token::from(b'='), Token::ASSIGN);
    assert_eq!(Token::from(b'+'), Token::PLUS);
    assert_eq!(Token::from(b','), Token::COMMA);
    assert_eq!(Token::from(b';'), Token::SEMICOLON);
    assert_eq!(Token::from(b'('), Token::LPAREN);
    assert_eq!(Token::from(b')'), Token::RPAREN);
    assert_eq!(Token::from(b'{'), Token::LBRACE);
    assert_eq!(Token::from(b'}'), Token::RBRACE);
    assert_eq!(Token::from(0), Token::EOF);
    assert_eq!(Token::from(b' '), Token::ILLEGAL);
}

#[test]
fn test_next_token() {
    use super::{Lexer, Token};

    let input = "=+(){},;";

    let expected_tokens = vec![
        Token::ASSIGN,
        Token::PLUS,
        Token::LPAREN,
        Token::RPAREN,
        Token::LBRACE,
        Token::RBRACE,
        Token::COMMA,
        Token::SEMICOLON,
        Token::EOF,
    ];

    let mut lexer = Lexer::new(input.into());

    for i in 0..=input.len() {
        let received_token = lexer.next_token();
        assert_eq!(received_token, expected_tokens[i]);
    }
}

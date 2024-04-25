#[test]
fn individual_token_test() {
    use super::Token;
    // Operators and delimiters
    assert_eq!(Token::from(b'='), Token::ASSIGN);
    assert_eq!(Token::from(b'+'), Token::PLUS);
    assert_eq!(Token::from(b'-'), Token::MINUS);
    assert_eq!(Token::from(b'!'), Token::BANG);
    assert_eq!(Token::from(b','), Token::COMMA);
    assert_eq!(Token::from(b';'), Token::SEMICOLON);
    assert_eq!(Token::from(b'('), Token::LPAREN);
    assert_eq!(Token::from(b'>'), Token::GREATERTHAN);
    assert_eq!(Token::from(b'<'), Token::LESSTHAN);
    assert_eq!(Token::from(b'/'), Token::SLASH);
    assert_eq!(Token::from(b'*'), Token::ASTERISK);
    assert_eq!(Token::from(b')'), Token::RPAREN);
    assert_eq!(Token::from(b'{'), Token::LSQUIRLY);
    assert_eq!(Token::from(b'}'), Token::RSQUIRLY);
    assert_eq!(Token::from(0), Token::EOF);
    assert_eq!(Token::from(b' '), Token::ILLEGAL);
    // Keywords and identifiers
    assert_eq!(Token::from("==".to_string()), Token::EQUAL);
    assert_eq!(Token::from("!=".to_string()), Token::NOTEQUAL);
    assert_eq!(Token::from("true".to_string()), Token::TRUE);
    assert_eq!(Token::from("false".to_string()), Token::FALSE);
    assert_eq!(Token::from("return".to_string()), Token::RETURN);
    assert_eq!(Token::from("if".to_string()), Token::IF);
    assert_eq!(Token::from("else".to_string()), Token::ELSE);
    assert_eq!(Token::from("fn".to_string()), Token::FUNCTION);
    assert_eq!(Token::from("let".to_string()), Token::LET);
    assert_eq!(
        Token::from("1234".to_string()),
        Token::INT("1234".to_string())
    );
    assert_eq!(
        Token::from("my_var".to_string()),
        Token::IDENT("my_var".to_string())
    );
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
        Token::LSQUIRLY,
        Token::RSQUIRLY,
        Token::COMMA,
        Token::SEMICOLON,
        Token::EOF,
    ];
    let mut lexer = Lexer::new(input.into());
    for i in 0..=input.len() {
        let received = lexer.next_token();
        assert_eq!(received, expected_tokens[i]);
    }
}

#[test]
fn full_code_tokenization() {
    use super::{Lexer, Token};
    let input = "
    let five = 5;
    let ten = 10;

    let add = fn(x, y) {
        x + y;
    };

    let result = add(five, ten);
    !-/*5;
    5 < 10 > 5;

    if (5 < 10) {
      return true;
    } else {
      return false;
    }

    10 == 10;
    10 != 9;
    ";
    let expected_tokens = vec![
        Token::LET,
        Token::IDENT("five".into()),
        Token::ASSIGN,
        Token::INT("5".into()),
        Token::SEMICOLON,
        Token::LET,
        Token::IDENT("ten".into()),
        Token::ASSIGN,
        Token::INT("10".into()),
        Token::SEMICOLON,
        Token::LET,
        Token::IDENT("add".into()),
        Token::ASSIGN,
        Token::FUNCTION,
        Token::LPAREN,
        Token::IDENT("x".into()),
        Token::COMMA,
        Token::IDENT("y".into()),
        Token::RPAREN,
        Token::LSQUIRLY,
        Token::IDENT("x".into()),
        Token::PLUS,
        Token::IDENT("y".into()),
        Token::SEMICOLON,
        Token::RSQUIRLY,
        Token::SEMICOLON,
        Token::LET,
        Token::IDENT("result".into()),
        Token::ASSIGN,
        Token::IDENT("add".into()),
        Token::LPAREN,
        Token::IDENT("five".into()),
        Token::COMMA,
        Token::IDENT("ten".into()),
        Token::RPAREN,
        Token::SEMICOLON,
        Token::BANG,
        Token::MINUS,
        Token::SLASH,
        Token::ASTERISK,
        Token::INT("5".into()),
        Token::SEMICOLON,
        Token::INT("5".into()),
        Token::LESSTHAN,
        Token::INT("10".into()),
        Token::GREATERTHAN,
        Token::INT("5".into()),
        Token::SEMICOLON,
        Token::IF,
        Token::LPAREN,
        Token::INT("5".into()),
        Token::LESSTHAN,
        Token::INT("10".into()),
        Token::RPAREN,
        Token::LSQUIRLY,
        Token::RETURN,
        Token::TRUE,
        Token::SEMICOLON,
        Token::RSQUIRLY,
        Token::ELSE,
        Token::LSQUIRLY,
        Token::RETURN,
        Token::FALSE,
        Token::SEMICOLON,
        Token::RSQUIRLY,
        Token::INT("10".into()),
        Token::EQUAL,
        Token::INT("10".into()),
        Token::SEMICOLON,
        Token::INT("10".into()),
        Token::NOTEQUAL,
        Token::INT("9".into()),
        Token::SEMICOLON,
        Token::EOF,
    ];
    let mut lexer = Lexer::new(input.into());
    for expected in expected_tokens {
        let received = lexer.next_token();
        println!("expected {:?}, got {:?}", expected, received);
        assert_eq!(received, expected);
    }
}

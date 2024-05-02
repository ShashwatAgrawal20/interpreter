#[test]
fn parser_creation() {
    use super::{Lexer, Parser};

    let input = "let bullshit = 69";
    let lexer = Lexer::new(input.into());
    let parser = Parser::new(lexer);
    assert_eq!(parser.curr_token, super::Token::LET);
    assert_eq!(
        parser.peek_token,
        super::Token::IDENT("bullshit".to_string())
    );
}

#[test]
fn shitty_let_parser_literal() {
    use super::{Lexer, Parser};
    use crate::ast::*;

    let input = "let bullshit = 69";
    let lexer = Lexer::new(input.into());
    let mut parser = Parser::new(lexer);

    let program_ast = parser.parse_program();
    assert_eq!(
        program_ast,
        AST {
            statements: vec![Statement::Let(LetStatement {
                name: Identifier {
                    value: "bullshit".to_string()
                },
                value: Expression::Literal(Literal {
                    value: "69".to_string()
                }),
            }),],
        }
    );
    // println!("{:#?}", program_ast);
    let input = "let bullshit = shit";
    let lexer = Lexer::new(input.into());
    let mut parser = Parser::new(lexer);

    let program_ast = parser.parse_program();
    assert_eq!(
        program_ast,
        AST {
            statements: vec![Statement::Let(LetStatement {
                name: Identifier {
                    value: "bullshit".to_string()
                },
                value: Expression::Identifier(Identifier {
                    value: "shit".to_string()
                }),
            }),],
        }
    );
}

#[test]
#[should_panic]
fn shitty_failing_test() {
    use super::{Lexer, Parser};

    let input = "let bullshit != shit";
    let lexer = Lexer::new(input.into());
    let mut parser = Parser::new(lexer);

    parser.parse_program();
}

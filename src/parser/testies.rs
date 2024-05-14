#[test]
fn parser_creation() {
    use super::{Lexer, Parser};

    let input = "let bullshit = 69;";
    let lexer = Lexer::new(input.into());
    let parser = Parser::new(lexer);
    assert_eq!(parser.curr_token, super::Token::LET);
    assert_eq!(
        parser.peek_token,
        super::Token::IDENT("bullshit".to_string())
    );
}

#[test]
fn parse_check_errors() {
    use super::{Lexer, Parser};
    let input = "let x 5; let = 10; let 838383;";
    let lexer = Lexer::new(input.into());
    let mut parser = Parser::new(lexer);

    let _program_ast = parser.parse_program();
    // println!("{:?}", parser.errors);
    assert_eq!(parser.errors.len(), 3)
}

#[test]
fn parse_let_statements() {
    use super::{Lexer, Parser};
    use crate::ast::*;

    let input = "let bullshit = 69;";
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
    let input = "let bullshit = shit;";
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
fn parse_return_statement() {
    use super::{Lexer, Parser};
    use crate::ast::*;

    let input = "return bullshit;";
    let lexer = Lexer::new(input.into());
    let mut parser = Parser::new(lexer);

    let program_ast = parser.parse_program();
    assert_eq!(
        program_ast,
        AST {
            statements: vec![Statement::Return(ReturnStatement {
                value: Expression::Identifier(Identifier {
                    value: "bullshit".to_string()
                }),
            }),],
        }
    );
    // println!("{:#?}", program_ast);
    let input = "return 69;";
    let lexer = Lexer::new(input.into());
    let mut parser = Parser::new(lexer);

    let program_ast = parser.parse_program();
    assert_eq!(
        program_ast,
        AST {
            statements: vec![Statement::Return(ReturnStatement {
                value: Expression::Literal(Literal {
                    value: "69".to_string()
                }),
            }),],
        }
    );
}

#[test]
fn parse_let_return_merge() {
    use super::{Lexer, Parser};
    use crate::ast::*;

    let input = "let bullshit = 69; return bullshit;";
    let lexer = Lexer::new(input.into());
    let mut parser = Parser::new(lexer);

    let program_ast = parser.parse_program();
    // println!("{:#?}", program_ast);
    assert_eq!(
        program_ast,
        AST {
            statements: vec![
                Statement::Let(LetStatement {
                    name: Identifier {
                        value: "bullshit".to_string()
                    },
                    value: Expression::Literal(Literal {
                        value: "69".to_string()
                    }),
                }),
                Statement::Return(ReturnStatement {
                    value: Expression::Identifier(Identifier {
                        value: "bullshit".to_string(),
                    }),
                }),
            ],
        }
    );
}

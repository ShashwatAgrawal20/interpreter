#[test]
fn test_shitty_ast() {
    use super::*;
    let identifier = Identifier {
        value: "a".to_string(),
    };
    let literal = Literal {
        value: "10".to_string(),
    };
    let expression_identifier = Expression::Identifier(identifier.clone());
    let expression_literal = Expression::Literal(literal);
    let let_statement = LetStatement {
        name: identifier.clone(),
        value: expression_literal.clone(),
    };
    let return_statement = ReturnStatement {
        value: expression_identifier.clone(),
    };
    let statements = vec![
        Statement::Let(let_statement),
        Statement::Return(return_statement),
    ];
    let ast = AST { statements };
    // println!("{:#?}", ast);
    assert_eq!(
        ast,
        AST {
            statements: vec![
                Statement::Let(LetStatement {
                    name: Identifier {
                        value: "a".to_string()
                    },
                    value: Expression::Literal(Literal {
                        value: "10".to_string()
                    }),
                }),
                Statement::Return(ReturnStatement {
                    value: Expression::Identifier(Identifier {
                        value: "a".to_string()
                    }),
                }),
            ],
        }
    );
}

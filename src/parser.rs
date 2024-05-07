use crate::ast::*;
use crate::lexer::{Lexer, Token};

mod testies;

pub struct Parser {
    lexer: Lexer,
    curr_token: Token,
    peek_token: Token,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Parser {
        let mut parser = Parser {
            lexer,
            curr_token: Token::EOF,
            peek_token: Token::EOF,
        };

        parser.next_token();
        parser.next_token();
        parser
    }

    fn next_token(&mut self) {
        self.curr_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    fn parse_program(&mut self) -> AST {
        let mut program_ast = AST {
            statements: Vec::new(),
        };

        while self.curr_token != Token::EOF {
            if let Some(statement) = self.parse_statement(self.curr_token.clone()) {
                program_ast.statements.push(statement);
            }
            self.next_token();
        }
        program_ast
    }

    fn parse_statement(&mut self, curr_token: Token) -> Option<Statement> {
        match curr_token {
            Token::LET => self.parse_let_statement(),
            Token::RETURN => self.parse_return_statement(),
            _ => None,
        }
    }

    fn parse_let_statement(&mut self) -> Option<Statement> {
        self.next_token();
        // println!("{:?}", &self.curr_token);
        let name = if let Token::IDENT(name) = &self.curr_token {
            Identifier {
                value: name.to_string(),
            }
        } else {
            eprintln!("expected Identifier, got {:?}", &self.curr_token);
            return None;
        };

        self.next_token();
        if self.curr_token != Token::ASSIGN {
            eprintln!("expected {:?}, got {:?}", Token::ASSIGN, self.curr_token);
            return None;
        }

        // shitty expression part starts here
        self.next_token();
        let value = match &self.curr_token {
            Token::INT(value) => Expression::Literal(Literal {
                value: value.to_string(),
            }),
            Token::IDENT(name) => Expression::Identifier(Identifier {
                value: name.to_string(),
            }),
            _ => return None,
        };

        Some(Statement::Let(LetStatement { name, value }))
    }

    fn parse_return_statement(&mut self) -> Option<Statement> {
        self.next_token();
        let value = match &self.curr_token {
            Token::INT(value) => Expression::Literal(Literal {
                value: value.to_string(),
            }),
            Token::IDENT(value) => Expression::Identifier(Identifier {
                value: value.to_string(),
            }),
            _ => return None,
        };
        Some(Statement::Return(ReturnStatement { value }))
    }
}

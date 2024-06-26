use crate::ast::*;
use crate::lexer::{Lexer, Token};

mod testies;

pub struct Parser {
    lexer: Lexer,
    curr_token: Token,
    peek_token: Token,
    pub errors: Vec<String>,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Parser {
        let mut parser = Parser {
            lexer,
            curr_token: Token::EOF,
            peek_token: Token::EOF,
            errors: vec![],
        };

        parser.next_token();
        parser.next_token();
        parser
    }

    fn has_semicolon(&mut self) -> bool {
        if self.peek_token != Token::SEMICOLON {
            self.errors
                .push(format!("expected SEMICOLON, got {:?}", self.peek_token));
            false
        } else {
            true
        }
    }

    fn next_token(&mut self) {
        self.curr_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    pub fn parse_program(&mut self) -> AST {
        let mut program_ast = AST {
            statements: Vec::new(),
        };

        while self.curr_token != Token::EOF {
            if let Some(statement) = self.parse_statement() {
                program_ast.statements.push(statement);
            }
            self.next_token();
        }
        program_ast
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        match &self.curr_token {
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
            self.errors
                .push(format!("expected IDENT, got {:?}", self.curr_token));
            return None;
        };

        self.next_token();
        if self.curr_token != Token::ASSIGN {
            self.errors
                .push(format!("expected ASSIGN, got {:?}", self.curr_token));
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
            _ => {
                self.errors
                    .push(format!("expected INT|IDENT, got {:?}", self.curr_token));
                return None;
            }
        };

        if !self.has_semicolon() {
            return None;
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
            _ => {
                self.errors
                    .push(format!("expected INT|IDENT, got {:?}", self.curr_token));
                return None;
            }
        };

        if !self.has_semicolon() {
            return None;
        };

        Some(Statement::Return(ReturnStatement { value }))
    }
}

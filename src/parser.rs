use crate::ast::*;
use crate::lexer::{Lexer, Token};

mod testies;

pub struct Parser {
    lexer: Lexer,
    cur_token: Token,
    peek_token: Token,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Parser {
        let mut parser = Parser {
            lexer,
            cur_token: Token::EOF,
            peek_token: Token::EOF,
        };

        parser.next_token();
        parser.next_token();
        parser
    }

    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    fn parse_program(&mut self) -> AST {
        let mut program_ast = AST {
            statements: Vec::new(),
        };

        while self.cur_token != Token::EOF {
            if let Some(statement) = self.parse_statement(self.cur_token.clone()) {
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
        todo!()
    }
    fn parse_return_statement(&mut self) -> Option<Statement> {
        todo!()
    }
}

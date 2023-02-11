mod statement;
mod util;

use crate::ast;
use crate::ast::{Expression, Statement};
use crate::error::Error;
use crate::lexer::Lexer;
use crate::token::{Token, TokenType};
use std::iter::Peekable;

struct Parser<'a> {
    lexer: Peekable<Lexer<'a>>,
}

impl<'a> Parser<'a> {
    fn new(lexer: Lexer<'a>) -> Self {
        let mut parser = Self {
            lexer: lexer.peekable(),
        };
        parser
    }

    // TODO: might be better to keep track of a set of errors
    fn parse_program(&mut self) -> Result<ast::Program, Error> {
        let mut program = ast::Program::new();

        while self.lexer.peek() != None {
            self.parse_statement()
                .map(|statement| program.statements.push(statement))?;
        }

        Ok(program)
    }
}
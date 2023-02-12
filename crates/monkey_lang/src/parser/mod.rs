mod expression;
mod statement;
mod util;

use crate::ast::{Expression, Program, Statement};
use crate::error::Error;
use crate::lexer::Lexer;
use std::iter::Peekable;

pub struct Parser<'a> {
    lexer: Peekable<Lexer<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        let mut parser = Self {
            lexer: lexer.peekable(),
        };
        parser
    }

    // TODO: might be better to keep track of a set of errors
    pub fn parse_program(&mut self) -> Result<Program, Error> {
        let mut program = Program::new();

        while self.lexer.peek() != None {
            self.parse_statement()
                .map(|statement| program.statements.push(statement))?;
        }

        Ok(program)
    }
}

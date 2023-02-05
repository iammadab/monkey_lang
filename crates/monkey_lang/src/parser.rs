use crate::ast;
use crate::ast::Statement;
use crate::lexer::Lexer;
use crate::token::{Token, TokenType};

struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Token,
    peek_token: Token,
}

impl<'a> Parser<'a> {
    fn new(lexer: Lexer<'a>) -> Self {
        let mut parser = Self {
            lexer,
            current_token: Lexer::new_eof_token(),
            peek_token: Lexer::new_eof_token(),
        };
        // read the current token and the peek token
        parser.next_token();
        parser.next_token();
        parser
    }

    fn next_token(&mut self) {
        self.current_token = self.peek_token.to_owned();
        self.peek_token = self.lexer.next_token();
    }

    fn parse_program(&mut self) -> ast::Program {
        let mut program = ast::Program::new();
        // TODO: impl better comparison
        //  better still, get rid of the while loop
        while !matches!(self.current_token.variant, TokenType::EOF) {
            self.parse_statement()
                .map(|statement| program.statements.push(statement));
            self.next_token();
        }

        program
    }

    // TODO: probably return better error type instead of string
    fn parse_statement(&mut self) -> Result<Box<dyn Statement>, String> {
        match self.current_token.variant {
            TokenType::LET => self.parse_let_statement(),
            _ => Err("unexpected token type while parsing statement".to_string()),
        }
    }

    fn parse_let_statement(&mut self) -> Result<Box<dyn Statement>, String> {
        // TODO: refactor to be more idiomatic
        //  maybe figure out a way to consume the tokens
        let let_token = self.current_token.clone();

        // peek and advance
        if self.peek_token.variant != TokenType::IDENT {
            return Err("expected identifier".to_string());
        }
        self.next_token();

        let identifier = ast::Identifier::new(
            self.current_token.clone(),
            self.current_token.literal.clone(),
        );
        // TODO: get rid of this
        let identifier_two = ast::Identifier::new(
            self.current_token.clone(),
            self.current_token.literal.clone(),
        );

        // peek and advance
        if self.peek_token.variant != TokenType::ASSIGN {
            return Err("expected assign token".to_string());
        }
        self.next_token();

        // advance until we hit a semi colon
        while self.current_token.variant != TokenType::SEMICOLON {
            self.next_token();
        }

        // reached the end build the let statement
        let let_statement = ast::LetStatement::new(let_token, identifier, Box::new(identifier_two));

        // TODO: two box, this cannot be good
        Ok(Box::new(let_statement))
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::{Program, Statement};
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    #[test]
    fn parse_let_statements() {
        let input = "let x = 5;\
        let y = 10;\
        let foobar = 838383;";

        let lexer = Lexer::new(input.chars());
        let mut parser = Parser::new(lexer);
        let program: Program = parser.parse_program();

        if program.statements.is_empty() {
            panic!("program statements should not be empty");
        }
        if program.statements.len() != 3 {
            panic!("program should have exactly 3 statements");
        }

        // TODO: assert the actual statements
        // TODO: assert the token literal also, whatever that means
    }
}

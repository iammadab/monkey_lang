use crate::ast;
use crate::ast::Statement;
use crate::lexer::Lexer;
use crate::token::{Token, TokenType};

// TODO: use a peekable lexer, get rid of current_token and peek_token
struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Option<Token>,
    peek_token: Option<Token>,
}

impl<'a> Parser<'a> {
    fn new(lexer: Lexer<'a>) -> Self {
        let mut parser = Self {
            lexer,
            current_token: None,
            peek_token: None,
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
        while self.current_token != None {
            self.parse_statement()
                .map(|statement| program.statements.push(statement));
            self.next_token();
        }

        program
    }

    // TODO: probably return better error type instead of string
    fn parse_statement(&mut self) -> Result<Box<dyn Statement>, String> {
        // TODO: get rid of unwrap
        match self.current_token.as_ref().unwrap().variant {
            TokenType::LET => self.parse_let_statement(),
            _ => Err("unexpected token type while parsing statement".to_string()),
        }
    }

    fn parse_let_statement(&mut self) -> Result<Box<dyn Statement>, String> {
        // TODO: refactor to be more idiomatic
        //  maybe figure out a way to consume the tokens
        let let_token = self.current_token.clone();

        // peek and advance
        if self.peek_token.as_ref().unwrap().variant != TokenType::IDENT {
            return Err("expected identifier".to_string());
        }
        self.next_token();

        let current_token_clone = self.current_token.clone();
        let current_token_clone_1 = self.current_token.clone();
        let current_token_clone_2 = self.current_token.clone();
        let current_token_clone_3 = self.current_token.clone();

        let identifier = ast::Identifier::new(
            // TODO: get rid of unwrap
            current_token_clone.unwrap(),
            current_token_clone_1.unwrap().literal,
        );
        // TODO: get rid of this
        let identifier_two = ast::Identifier::new(
            current_token_clone_2.unwrap(),
            current_token_clone_3.unwrap().literal,
        );

        // peek and advance
        if self.peek_token.as_ref().unwrap().variant != TokenType::ASSIGN {
            return Err("expected assign token".to_string());
        }
        self.next_token();

        // advance until we hit a semi colon
        while self.current_token.as_ref().unwrap().variant != TokenType::SEMICOLON {
            self.next_token();
        }

        // reached the end build the let statement
        let let_statement =
            ast::LetStatement::new(let_token.unwrap(), identifier, Box::new(identifier_two));

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

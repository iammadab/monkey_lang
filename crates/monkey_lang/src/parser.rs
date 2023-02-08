use crate::ast;
use crate::ast::Statement;
use crate::lexer::Lexer;
use crate::token::{Token, TokenType};
use std::iter::Peekable;

// TODO: Implement better error handling

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

    fn peek_token(&mut self) -> Option<&Token> {
        self.lexer.peek()
    }

    fn next_token(&mut self) -> Option<Token> {
        self.lexer.next()
    }

    fn expect_peek(&mut self, expected_token_variant: TokenType) -> Result<Token, String> {
        if let Some(peek_token) = self.peek_token() {
            if peek_token.variant != expected_token_variant {
                // TODO: return a better error message
                Err("expected a different token".to_string())
            } else {
                // we want to return the actual token
                self.next_token()
                    .ok_or("confirmed it is the expected token type".to_string())
            }
        } else {
            // TODO: return a better error message
            Err("expected a different token".to_string())
        }
    }

    fn parse_program(&mut self) -> Result<ast::Program, String> {
        let mut program = ast::Program::new();

        // TODO: can we get rid of the while loop?
        while self.lexer.peek() != None {
            self.parse_statement()
                .map(|statement| program.statements.push(statement))?;
        }

        Ok(program)
    }

    fn parse_statement(&mut self) -> Result<Box<dyn Statement>, String> {
        // TODO: can this be done without the if statement
        if let Some(peek_token) = self.peek_token() {
            match peek_token.variant {
                TokenType::LET => self.parse_let_statement(),
                _ => Err("unexpected token type while parsing statement".to_string()),
            }
        } else {
            Err("unexpected token type while parsing statement".to_string())
        }
    }

    fn parse_let_statement(&mut self) -> Result<Box<dyn Statement>, String> {
        // this tries to parse a let statement returning errors as it sees fit
        let let_token = self.next_token();
        // how do we do this
        // the next token should be an i
        todo!()
    }

    /*
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
    */
}

#[cfg(test)]
mod tests {
    use crate::ast::{Program, Statement};
    use crate::lexer::Lexer;
    use crate::parser::Parser;
    use crate::token::{Token, TokenType};

    #[test]
    fn expect_peek() {
        let input = "x = 5";
        let lexer = Lexer::new(input.chars());
        let mut parser = Parser::new(lexer);

        // error condition
        // TODO: change test case once you update how errors are handled
        assert_eq!(
            parser.expect_peek(TokenType::ASSIGN),
            Err("expected a different token".to_string())
        );

        assert_eq!(
            parser.expect_peek(TokenType::IDENT),
            Ok(Token::new(TokenType::IDENT, "x"))
        );
    }

    #[test]
    fn parse_let_statements() {
        let input = "let x = 5;\
        let y = 10;\
        let foobar = 838383;";

        let lexer = Lexer::new(input.chars());
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program().expect("should parse successfully");

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

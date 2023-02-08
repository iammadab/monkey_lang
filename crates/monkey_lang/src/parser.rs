use crate::ast;
use crate::ast::{Expression, Statement};
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

    fn next_token(&mut self) -> Result<Token, String> {
        if let Some(token) = self.lexer.next() {
            Ok(token)
        } else {
            Err("no more tokens".to_string())
        }
    }

    fn expect_next_token(&mut self, expected_token_variant: TokenType) -> Result<Token, String> {
        if let Some(peek_token) = self.peek_token() {
            if peek_token.variant != expected_token_variant {
                // TODO: return a better error message
                Err("expected a different token".to_string())
            } else {
                // we want to return the actual token
                self.next_token()
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

    fn parse_statement(&mut self) -> Result<Statement, String> {
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

    /// Parses statements of the form:
    /// let <identifier> = <expression>;
    fn parse_let_statement(&mut self) -> Result<Statement, String> {
        self.expect_next_token(TokenType::LET)?;

        let identifier_token = self.expect_next_token(TokenType::IDENT)?;

        self.expect_next_token(TokenType::ASSIGN)?;

        // skip all expressions until we hit a semi colon
        // as we are not handling expressions yet
        while self.next_token()?.variant != TokenType::SEMICOLON {
            // do nothing
        }

        // build the let statement
        Ok(Statement::Let {
            name: identifier_token.literal,
            value: Expression::Identifier("".to_string()),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::{Expression, Statement};
    use crate::lexer::Lexer;
    use crate::parser::Parser;
    use crate::token::{Token, TokenType};

    #[test]
    fn expect_next_token() {
        let input = "x = 5";
        let lexer = Lexer::new(input.chars());
        let mut parser = Parser::new(lexer);

        // error condition
        // TODO: change test case once you update how errors are handled
        assert_eq!(
            parser.expect_next_token(TokenType::ASSIGN),
            Err("expected a different token".to_string())
        );

        assert_eq!(
            parser.expect_next_token(TokenType::IDENT),
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

        assert_eq!(
            program.statements[0],
            Statement::Let {
                name: "x".to_string(),
                value: Expression::Identifier("".to_string())
            }
        );
        assert_eq!(
            program.statements[1],
            Statement::Let {
                name: "y".to_string(),
                value: Expression::Identifier("".to_string())
            }
        );
        assert_eq!(
            program.statements[2],
            Statement::Let {
                name: "foobar".to_string(),
                value: Expression::Identifier("".to_string())
            }
        );
    }
}

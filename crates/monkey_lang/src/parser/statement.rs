use crate::ast::{Expression, Statement};
use crate::error::Error;
use crate::parser::util::Precedence;
use crate::parser::Parser;
use crate::token::TokenType;
use std::os::macos::raw::stat;

impl<'a> Parser<'a> {
    pub(crate) fn parse_statement(&mut self) -> Result<Statement, Error> {
        if let Some(peek_token) = self.peek_token() {
            match peek_token.variant {
                TokenType::LET => self.parse_let_statement(),
                TokenType::RETURN => self.parse_return_statement(),
                TokenType::LEFTBRACE => self.parse_block_statement(),
                _ => self.parse_expression_statement(),
            }
        } else {
            Err(Error::MissingToken)
        }
    }

    /// Parses statements of the form:
    /// let <identifier> = <expression>;
    fn parse_let_statement(&mut self) -> Result<Statement, Error> {
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

    /// Parses statements of the form:
    /// let <identifier> = <expression>;
    fn parse_return_statement(&mut self) -> Result<Statement, Error> {
        self.expect_next_token(TokenType::RETURN)?;

        // skip all expressions until we hit a semi colon
        // as we are not handling expressions yet
        while self.next_token()?.variant != TokenType::SEMICOLON {
            // do nothing
        }

        // build the return statement
        Ok(Statement::Return {
            return_value: Expression::Identifier("".to_string()),
        })
    }

    /// Parses expressions, return them as an expression statement
    fn parse_expression_statement(&mut self) -> Result<Statement, Error> {
        let expression = self.parse_expression(Precedence::LOWEST)?;
        // TODO: handle semicolon
        self.optional_expect_next_token(TokenType::SEMICOLON);
        Ok(Statement::Expression(expression))
    }

    /// Parses statements of the form
    /// { a; b; c; } where a, b, c are other statements
    fn parse_block_statement(&mut self) -> Result<Statement, Error> {
        self.expect_next_token(TokenType::LEFTBRACE)?;

        let mut statements = Vec::new();

        // keep parsing statements until we reach a right brace
        // TODO: take into account eof, do we need to handle that?
        // TODO: need something that checks for a token or an end token
        while self.expect_next_token(TokenType::RIGHTBRACE).is_err() {
            statements.push(Box::new(self.parse_statement()?));
        }

        Ok(Statement::Block { statements })
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::{Expression, Statement};
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    // TODO: the tests here should parse statements not programs
    //  refactor

    #[test]
    fn parse_let_statements() {
        let input = "let x = 5;\
        let y = 10;\
        let foobar = 838383;";

        let lexer = Lexer::new(input.chars());
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program().unwrap();

        assert_eq!(program.statements.len(), 3);

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

    #[test]
    fn parse_return_statements() {
        let input = "return 5;\
        return 10;\
        return 993322;";

        let lexer = Lexer::new(input.chars());
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program().unwrap();

        assert_eq!(program.statements.len(), 3);
        assert_eq!(
            program.statements[0],
            Statement::Return {
                return_value: Expression::Identifier("".to_string())
            }
        );
        assert_eq!(
            program.statements[1],
            Statement::Return {
                return_value: Expression::Identifier("".to_string())
            }
        );
        assert_eq!(
            program.statements[2],
            Statement::Return {
                return_value: Expression::Identifier("".to_string())
            }
        );
    }

    #[test]
    fn parse_expression_statements() {
        let input = "3 + 4; -5 * 5;";
        let lexer = Lexer::new(input.chars());
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program().unwrap();

        assert_eq!(program.statements.len(), 2);
        assert_eq!(
            program.statements[0],
            Statement::Expression(Expression::Infix {
                left: Box::new(Expression::IntegerLiteral(3)),
                operator: "+".to_string(),
                right: Box::new(Expression::IntegerLiteral(4))
            })
        );
        assert_eq!(
            program.statements[1],
            Statement::Expression(Expression::Infix {
                left: Box::new(Expression::Prefix {
                    operator: "-".to_string(),
                    right: Box::new(Expression::IntegerLiteral(5))
                }),
                operator: "*".to_string(),
                right: Box::new(Expression::IntegerLiteral(5))
            })
        );
    }

    #[test]
    fn parse_block_statement() {
        let input = "{ x }";
        let lexer = Lexer::new(input.chars());
        let mut parser = Parser::new(lexer);
        let statement = parser.parse_statement().unwrap();

        assert_eq!(
            statement,
            Statement::Block {
                statements: vec![Box::new(Statement::Expression(Expression::Identifier(
                    "x".to_string()
                )))]
            }
        );
    }
}

use crate::ast::{Block, Expression, Statement};
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

        let expression = self.parse_expression(Precedence::LOWEST)?;

        self.expect_next_token(TokenType::SEMICOLON)?;

        Ok(Statement::Let {
            name: identifier_token.literal,
            value: expression,
        })
    }

    /// Parses statements of the form:
    /// let <identifier> = <expression>;
    fn parse_return_statement(&mut self) -> Result<Statement, Error> {
        self.expect_next_token(TokenType::RETURN)?;

        let expression = self.parse_expression(Precedence::LOWEST)?;

        self.expect_next_token(TokenType::SEMICOLON)?;

        Ok(Statement::Return {
            return_value: expression,
        })
    }

    /// Parses expressions, return them as an expression statement
    fn parse_expression_statement(&mut self) -> Result<Statement, Error> {
        let expression = self.parse_expression(Precedence::LOWEST)?;
        // TODO: handle semicolon
        self.optional_expect_next_token(TokenType::SEMICOLON);
        Ok(Statement::Expression(expression))
    }

    // TODO: move this to a different file
    /// Parses statements of the form
    /// { a; b; c; } where a, b, c are other statements
    pub(crate) fn parse_block(&mut self) -> Result<Block, Error> {
        self.expect_next_token(TokenType::LEFTBRACE)?;

        let mut statements = Vec::new();

        // keep parsing statements until we reach a right brace
        // TODO: take into account eof, do we need to handle that?
        // TODO: need something that checks for a token or an end token
        while self.expect_next_token(TokenType::RIGHTBRACE).is_err() {
            statements.push(self.parse_statement()?);
        }

        Ok(Block { statements })
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::{Block, Expression, InfixOperator, PrefixOperator, Statement};
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
                value: Expression::IntegerLiteral(5)
            }
        );
        assert_eq!(
            program.statements[1],
            Statement::Let {
                name: "y".to_string(),
                value: Expression::IntegerLiteral(10)
            }
        );
        assert_eq!(
            program.statements[2],
            Statement::Let {
                name: "foobar".to_string(),
                value: Expression::IntegerLiteral(838383)
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
                return_value: Expression::IntegerLiteral(5)
            }
        );
        assert_eq!(
            program.statements[1],
            Statement::Return {
                return_value: Expression::IntegerLiteral(10)
            }
        );
        assert_eq!(
            program.statements[2],
            Statement::Return {
                return_value: Expression::IntegerLiteral(993322)
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
                operator: InfixOperator::PLUS,
                right: Box::new(Expression::IntegerLiteral(4))
            })
        );
        assert_eq!(
            program.statements[1],
            Statement::Expression(Expression::Infix {
                left: Box::new(Expression::Prefix {
                    operator: PrefixOperator::NEGATE,
                    right: Box::new(Expression::IntegerLiteral(5))
                }),
                operator: InfixOperator::MULTIPLY,
                right: Box::new(Expression::IntegerLiteral(5))
            })
        );
    }

    #[test]
    fn parse_block() {
        let input = "{ x; 2 + 3; let a = 5; }";
        let lexer = Lexer::new(input.chars());
        let mut parser = Parser::new(lexer);
        let statement = parser.parse_block().unwrap();

        assert_eq!(
            statement,
            Block {
                statements: vec![
                    Statement::Expression(Expression::Identifier("x".to_string())),
                    Statement::Expression(Expression::Infix {
                        left: Box::new(Expression::IntegerLiteral(2)),
                        operator: InfixOperator::PLUS,
                        right: Box::new(Expression::IntegerLiteral(3))
                    }),
                    Statement::Let {
                        name: "a".to_string(),
                        value: Expression::IntegerLiteral(5)
                    }
                ]
            }
        );
    }
}

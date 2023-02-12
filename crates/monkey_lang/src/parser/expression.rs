use crate::ast::Expression;
use crate::error::Error;
use crate::parser::util::Precedence;
use crate::parser::Parser;
use crate::token::{Token, TokenType};
use std::cmp::Ordering;

impl<'a> Parser<'a> {
    /// Implementation of the pratt parsing technique
    pub(crate) fn parse_expression(
        &mut self,
        left_precedence: Precedence,
    ) -> Result<Expression, Error> {
        let mut left_expression = self.parse_null_definition()?;

        let (mut peek_token, mut token_at_end) = self.peek_token_return_end_status();

        while !token_at_end
            && left_precedence < Precedence::get_precedence(&peek_token.unwrap().variant)
        {
            left_expression = self.parse_infix_expression(left_expression)?;
            (peek_token, token_at_end) = self.peek_token_return_end_status();
        }

        Ok(left_expression)
    }

    /// Null defintitions are expressions that have nothing to the left of them
    /// e.g. an identifier
    fn parse_null_definition(&mut self) -> Result<Expression, Error> {
        if let Some(peek_token) = self.peek_token() {
            match peek_token.variant {
                TokenType::IDENT => self.parse_identifier(),
                TokenType::INT => self.parse_integer_literal(),
                TokenType::BANG => self.parse_prefix_expression(),
                TokenType::MINUS => self.parse_prefix_expression(),
                TokenType::TRUE => self.parse_boolean_expression(),
                TokenType::FALSE => self.parse_boolean_expression(),
                TokenType::LEFTPAREN => self.parse_grouped_expression(),
                TokenType::IF => self.parse_if_expression(),
                TokenType::FUNCTION => self.parse_function_literal_expression(),
                _ => Err(Error::UnexpectedToken(peek_token.literal.clone())),
            }
        } else {
            Err(Error::MissingToken)
        }
    }

    /// Builds an AST out of an identifier token
    fn parse_identifier(&mut self) -> Result<Expression, Error> {
        let identifier_token = self.expect_next_token(TokenType::IDENT)?;
        Ok(Expression::Identifier(identifier_token.literal))
    }

    /// Builds an AST out of an integer token
    fn parse_integer_literal(&mut self) -> Result<Expression, Error> {
        let int_token = self.expect_next_token(TokenType::INT)?;
        // need to convert the integer value to an actual integer value
        let int_value: i64 = int_token
            .literal
            .parse()
            .map_err(|_| Error::InvalidIntegerValue(int_token.literal))?;
        Ok(Expression::IntegerLiteral(int_value))
    }

    /// Builds an AST out of a prefix expression
    /// e.g. -5 or !true
    fn parse_prefix_expression(&mut self) -> Result<Expression, Error> {
        let prefix_token = self.next_token()?;
        let right_expression = self.parse_expression(Precedence::PREFIX)?;
        Ok(Expression::Prefix {
            operator: prefix_token.literal,
            right: Box::new(right_expression),
        })
    }

    /// Builds an AST out of an infix expression
    /// e.g. 5 + 5
    fn parse_infix_expression(&mut self, left_expression: Expression) -> Result<Expression, Error> {
        let operator_token = self.next_token()?;
        let operator_precedence = Precedence::get_precedence(&operator_token.variant);
        let right_expression = self.parse_expression(operator_precedence)?;

        Ok(Expression::Infix {
            left: Box::new(left_expression),
            operator: operator_token.literal,
            right: Box::new(right_expression),
        })
    }

    /// Builds an AST out of a boolean expression
    fn parse_boolean_expression(&mut self) -> Result<Expression, Error> {
        // try to extract true first, if that fails try false
        let boolean_token = self
            .expect_next_token(TokenType::TRUE)
            .or_else(|_| self.expect_next_token(TokenType::FALSE))?;

        let bool_value = match boolean_token.literal.as_str() {
            "true" => true,
            "false" => false,
            _ => Err(Error::InvalidBooleanValue(boolean_token.literal.clone()))?,
        };

        Ok(Expression::Boolean(bool_value))
    }

    /// Parses grouped expression by bumping up the precedence for group
    /// expressions
    fn parse_grouped_expression(&mut self) -> Result<Expression, Error> {
        self.expect_next_token(TokenType::LEFTPAREN)?;
        // take as many tokens as we can until we hit the right paren
        // it will break at right paren, because precedence value for
        // right paren is also lowest
        // condition for continuation is left_precedence < right_precedence
        // lowest !< lowest hence the break
        let grouped_expression = self.parse_expression(Precedence::LOWEST)?;
        self.expect_next_token(TokenType::RIGHTPAREN)?;
        Ok(grouped_expression)
    }

    /// Builds an AST for If statements, with an optional else block
    fn parse_if_expression(&mut self) -> Result<Expression, Error> {
        self.expect_next_token(TokenType::IF)?;

        let condition = Box::new(self.parse_expression(Precedence::LOWEST)?);
        let consequence = self.parse_block()?;
        let alternative = if self.expect_next_token(TokenType::ELSE).is_ok() {
            Some(self.parse_block()?)
        } else {
            None
        };

        Ok(Expression::If {
            condition,
            consequence,
            alternative,
        })
    }

    /// Builds an AST for a function literaal expressoin
    fn parse_function_literal_expression(&mut self) -> Result<Expression, Error> {
        self.expect_next_token(TokenType::FUNCTION)?;
        self.expect_next_token(TokenType::LEFTPAREN)?;

        let mut parameters = Vec::new();
        while self.expect_next_token(TokenType::RIGHTPAREN).is_err() {
            let identifier_expression = self.parse_identifier()?;
            parameters.push(identifier_expression.to_string());

            // TODO: possibility of not enforcing commas here??
            self.optional_expect_next_token(TokenType::COMMA);
        }

        let body = self.parse_block()?;

        Ok(Expression::FunctionLiteral { parameters, body })
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::{Block, Expression, Statement};
    use crate::lexer::Lexer;
    use crate::parser::util::Precedence;
    use crate::parser::Parser;

    #[test]
    fn parse_identifier_expression() {
        let input = "foobar;";
        let lexer = Lexer::new(input.chars());
        let mut parser = Parser::new(lexer);

        let expression = parser.parse_expression(Precedence::default()).unwrap();

        assert_eq!(expression, Expression::Identifier("foobar".to_string()));
    }

    #[test]
    fn parse_integer_expression() {
        let input = "5;";
        let lexer = Lexer::new(input.chars());
        let mut parser = Parser::new(lexer);

        let expression = parser.parse_expression(Precedence::default()).unwrap();

        assert_eq!(expression, Expression::IntegerLiteral(5));
    }

    #[test]
    fn parse_boolean_expression() {
        let input = "true";
        let lexer = Lexer::new(input.chars());
        let mut parser = Parser::new(lexer);
        let expression = parser.parse_expression(Precedence::default()).unwrap();
        assert_eq!(expression, Expression::Boolean(true));

        let input = "false";
        let lexer = Lexer::new(input.chars());
        let mut parser = Parser::new(lexer);
        let expression = parser.parse_expression(Precedence::default()).unwrap();
        assert_eq!(expression, Expression::Boolean(false));
    }

    #[test]
    fn parse_if_expression() {
        let input = "if (x < y) { x }  else { y }";
        let lexer = Lexer::new(input.chars());
        let mut parser = Parser::new(lexer);
        let expression = parser.parse_expression(Precedence::default()).unwrap();

        assert_eq!(
            expression,
            Expression::If {
                condition: Box::new(Expression::Infix {
                    left: Box::new(Expression::Identifier("x".to_string())),
                    operator: "<".to_string(),
                    right: Box::new(Expression::Identifier("y".to_string()))
                }),
                consequence: Block {
                    statements: vec![Statement::Expression(Expression::Identifier(
                        "x".to_string()
                    ))]
                },
                alternative: Some(Block {
                    statements: vec![Statement::Expression(Expression::Identifier(
                        "y".to_string()
                    ))]
                })
            }
        )
    }

    #[test]
    fn parse_function_literal() {
        let input = "fn(x, y) {x + y;}";
        let lexer = Lexer::new(input.chars());
        let mut parser = Parser::new(lexer);
        let expression = parser.parse_expression(Precedence::default()).unwrap();

        assert_eq!(
            expression,
            Expression::FunctionLiteral {
                parameters: vec!["x".to_string(), "y".to_string()],
                body: Block {
                    statements: vec![Statement::Expression(Expression::Infix {
                        left: Box::new(Expression::Identifier("x".to_string())),
                        operator: "+".to_string(),
                        right: Box::new(Expression::Identifier("y".to_string()))
                    })]
                }
            }
        );

        let input = "fn() {\
        let a = 2;\
        let b = a + 1;\
        }";
        let lexer = Lexer::new(input.chars());
        let mut parser = Parser::new(lexer);
        let expression = parser.parse_expression(Precedence::default()).unwrap();

        assert_eq!(
            expression,
            Expression::FunctionLiteral {
                parameters: Vec::new(),
                body: Block {
                    statements: vec![
                        Statement::Let {
                            name: "a".to_string(),
                            value: Expression::IntegerLiteral(2),
                        },
                        Statement::Let {
                            name: "b".to_string(),
                            value: Expression::Infix {
                                left: Box::new(Expression::Identifier("a".to_string())),
                                operator: "+".to_string(),
                                right: Box::new(Expression::IntegerLiteral(1))
                            }
                        }
                    ]
                }
            }
        );

        let input = "fn() {}";
        let lexer = Lexer::new(input.chars());
        let mut parser = Parser::new(lexer);
        let expression = parser.parse_expression(Precedence::default()).unwrap();

        assert_eq!(
            expression,
            Expression::FunctionLiteral {
                parameters: Vec::new(),
                body: Block {
                    statements: Vec::new()
                }
            }
        );

        let input = "fn(x) {}";
        let lexer = Lexer::new(input.chars());
        let mut parser = Parser::new(lexer);
        let expression = parser.parse_expression(Precedence::default()).unwrap();

        assert_eq!(
            expression,
            Expression::FunctionLiteral {
                parameters: vec!["x".to_string()],
                body: Block {
                    statements: Vec::new()
                }
            }
        );

        let input = "fn(x, y, z) {}";
        let lexer = Lexer::new(input.chars());
        let mut parser = Parser::new(lexer);
        let expression = parser.parse_expression(Precedence::default()).unwrap();

        assert_eq!(
            expression,
            Expression::FunctionLiteral {
                parameters: vec!["x".to_string(), "y".to_string(), "z".to_string()],
                body: Block {
                    statements: Vec::new()
                }
            }
        );
    }

    #[test]
    fn parse_prefix_expressions() {
        let input = "!wanted;";
        let lexer = Lexer::new(input.chars());
        let mut parser = Parser::new(lexer);

        let expression = parser.parse_expression(Precedence::default()).unwrap();
        assert_eq!(
            expression,
            Expression::Prefix {
                operator: "!".to_string(),
                right: Box::new(Expression::Identifier("wanted".to_string()))
            }
        );

        let input = "-15;";
        let lexer = Lexer::new(input.chars());
        let mut parser = Parser::new(lexer);

        let expression = parser.parse_expression(Precedence::default()).unwrap();
        assert_eq!(
            expression,
            Expression::Prefix {
                operator: "-".to_string(),
                right: Box::new(Expression::IntegerLiteral(15))
            }
        );

        let input = "!true;";
        let lexer = Lexer::new(input.chars());
        let mut parser = Parser::new(lexer);

        let expression = parser.parse_expression(Precedence::default()).unwrap();
        assert_eq!(
            expression,
            Expression::Prefix {
                operator: "!".to_string(),
                right: Box::new(Expression::Boolean(true))
            }
        );

        let input = "!false";
        let lexer = Lexer::new(input.chars());
        let mut parser = Parser::new(lexer);

        let expression = parser.parse_expression(Precedence::default()).unwrap();
        assert_eq!(
            expression,
            Expression::Prefix {
                operator: "!".to_string(),
                right: Box::new(Expression::Boolean(false))
            }
        );
    }

    fn parse_expression_input(input: &str) -> String {
        let lexer = Lexer::new(input.chars());
        let mut parser = Parser::new(lexer);
        let expression = parser.parse_expression(Precedence::default()).unwrap();
        expression.to_string()
    }

    #[test]
    fn parse_infix_expressions() {
        let input = "5 + 5;";
        assert_eq!(parse_expression_input(input), "(5 + 5)");

        let input = "5 - 5;";
        assert_eq!(parse_expression_input(input), "(5 - 5)");

        let input = "5 * 5;";
        assert_eq!(parse_expression_input(input), "(5 * 5)");

        let input = "5 / 5;";
        assert_eq!(parse_expression_input(input), "(5 / 5)");

        let input = "5 > 5;";
        assert_eq!(parse_expression_input(input), "(5 > 5)");

        let input = "5 < 5;";
        assert_eq!(parse_expression_input(input), "(5 < 5)");

        let input = "5 == 5;";
        assert_eq!(parse_expression_input(input), "(5 == 5)");

        let input = "5 != 5;";
        assert_eq!(parse_expression_input(input), "(5 != 5)");

        let input = "5 + 5 * 2 + 2;";
        assert_eq!(parse_expression_input(input), "((5 + (5 * 2)) + 2)");

        let input = "true == true";
        assert_eq!(parse_expression_input(input), "(true == true)");
    }

    #[test]
    fn operator_precedence_parsing() {
        let input = "-a * b";
        assert_eq!(parse_expression_input(input), "((-a) * b)");

        let input = "!-a";
        assert_eq!(parse_expression_input(input), "(!(-a))");

        let input = "a + b + c";
        assert_eq!(parse_expression_input(input), "((a + b) + c)");

        let input = "a + b - c";
        assert_eq!(parse_expression_input(input), "((a + b) - c)");

        let input = "a * b * c";
        assert_eq!(parse_expression_input(input), "((a * b) * c)");

        let input = "a * b / c";
        assert_eq!(parse_expression_input(input), "((a * b) / c)");

        let input = "a + b / c";
        assert_eq!(parse_expression_input(input), "(a + (b / c))");

        let input = "a + b * c + d / e - f";
        assert_eq!(
            parse_expression_input(input),
            "(((a + (b * c)) + (d / e)) - f)"
        );

        let input = "5 > 4 == 3 < 4";
        assert_eq!(parse_expression_input(input), "((5 > 4) == (3 < 4))");

        let input = "5 < 4 != 3 > 4";
        assert_eq!(parse_expression_input(input), "((5 < 4) != (3 > 4))");

        let input = "3 + 4 * 5 == 3 * 1 + 4 * 5";
        assert_eq!(
            parse_expression_input(input),
            "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))"
        );

        let input = "true";
        assert_eq!(parse_expression_input(input), "true");

        let input = "false";
        assert_eq!(parse_expression_input(input), "false");

        let input = "3 > 5 == false";
        assert_eq!(parse_expression_input(input), "((3 > 5) == false)");

        let input = "3 < 5 == true";
        assert_eq!(parse_expression_input(input), "((3 < 5) == true)");

        let input = "1 + (2 + 3) + 4";
        assert_eq!(parse_expression_input(input), "((1 + (2 + 3)) + 4)");

        let input = "(5 + 5) * 2";
        assert_eq!(parse_expression_input(input), "((5 + 5) * 2)");

        let input = "2 / (5 + 5)";
        assert_eq!(parse_expression_input(input), "(2 / (5 + 5))");

        let input = "-(5 + 5)";
        assert_eq!(parse_expression_input(input), "(-(5 + 5))");
    }
}

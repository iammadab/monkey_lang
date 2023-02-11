use crate::ast::Expression;
use crate::error::Error;
use crate::parser::util::Precedence;
use crate::parser::Parser;
use crate::token::{Token, TokenType};
use std::cmp::Ordering;

impl<'a> Parser<'a> {
    fn parse_expression(&mut self, left_precedence: Precedence) -> Result<Expression, Error> {
        // TODO: refactor this, heavily
        let mut left_expression = self.parse_null_definition()?;

        // if the peek token is none we should return the left expression
        let mut peek_token = self.peek_token();
        let semicolon = Token::new(TokenType::SEMICOLON, ";");
        let mut token_at_end = peek_token.is_none() || peek_token.unwrap() == &semicolon;
        if token_at_end {
            return Ok(left_expression);
        }
        let mut precedence = Precedence::get_precedence(&peek_token.unwrap().variant);

        // TODO: make semi colons optional
        while left_precedence < precedence && !token_at_end {
            left_expression = self.parse_infix_expression(left_expression)?;
            peek_token = self.peek_token();
            token_at_end = peek_token.is_none() || peek_token.unwrap() == &semicolon;
            if token_at_end {
                return Ok(left_expression);
            }
            precedence = Precedence::get_precedence(&peek_token.unwrap().variant);
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
}

#[cfg(test)]
mod tests {
    use crate::ast::Expression;
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
    }

    #[test]
    fn operator_precedence_parsing() {
        // TODO: remove semicolons from these tests
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

        // TODO: handle weird test
        // let input = "3 + 4; -5 * 5;";
        // assert_eq!(parse_expression_input(input), "(3 + 4)((-5) * 5)");

        let input = "5 > 4 == 3 < 4";
        assert_eq!(parse_expression_input(input), "((5 > 4) == (3 < 4))");

        let input = "5 < 4 != 3 > 4";
        assert_eq!(parse_expression_input(input), "((5 < 4) != (3 > 4))");

        let input = "3 + 4 * 5 == 3 * 1 + 4 * 5";
        assert_eq!(
            parse_expression_input(input),
            "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))"
        );
    }
}

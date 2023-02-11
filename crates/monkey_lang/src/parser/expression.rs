use crate::ast::Expression;
use crate::error::Error;
use crate::parser::Parser;
use crate::token::TokenType;

impl<'a> Parser<'a> {
    fn parse_expression(&mut self) -> Result<Expression, Error> {
        self.parse_null_definition()
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
        let right_expression = self.parse_expression()?;
        Ok(Expression::Prefix {
            operator: prefix_token.literal,
            right: Box::new(right_expression),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::Expression;
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    #[test]
    fn parse_identifier_expression() {
        let input = "foobar;";
        let lexer = Lexer::new(input.chars());
        let mut parser = Parser::new(lexer);

        let expression = parser.parse_expression().unwrap();

        assert_eq!(expression, Expression::Identifier("foobar".to_string()));
    }

    #[test]
    fn parse_integer_expression() {
        let input = "5;";
        let lexer = Lexer::new(input.chars());
        let mut parser = Parser::new(lexer);

        let expression = parser.parse_expression().unwrap();

        assert_eq!(expression, Expression::IntegerLiteral(5));
    }

    #[test]
    fn parse_prefix_expression() {
        let input = "!wanted";
        let lexer = Lexer::new(input.chars());
        let mut parser = Parser::new(lexer);

        let expression = parser.parse_expression().unwrap();
        assert_eq!(
            expression,
            Expression::Prefix {
                operator: "!".to_string(),
                right: Box::new(Expression::Identifier("wanted".to_string()))
            }
        );

        let input = "-15";
        let lexer = Lexer::new(input.chars());
        let mut parser = Parser::new(lexer);

        let expression = parser.parse_expression().unwrap();
        assert_eq!(
            expression,
            Expression::Prefix {
                operator: "-".to_string(),
                right: Box::new(Expression::IntegerLiteral(15))
            }
        );
    }
}

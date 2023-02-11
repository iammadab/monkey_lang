use crate::ast::Expression;
use crate::error::Error;
use crate::parser::Parser;
use crate::token::TokenType;

impl<'a> Parser<'a> {
    fn parse_expression(&mut self) -> Result<Expression, Error> {
        // nud - a token that has nothing to it's left e.g. identifiers, integer literals
        // based on the head token, we need to figure out if there is a nud
        // parsing function associated with it.
        // if there is, we should run the parsing function to get the nud
        self.parse_null_definition()
    }

    /// Null defintitions are expressions that have nothing to the left of them
    /// e.g. an identifier
    fn parse_null_definition(&mut self) -> Result<Expression, Error> {
        self.parse_identifier()
    }

    fn parse_identifier(&mut self) -> Result<Expression, Error> {
        let identifier_token = self.expect_next_token(TokenType::IDENT)?;
        Ok(Expression::Identifier(identifier_token.literal))
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
}

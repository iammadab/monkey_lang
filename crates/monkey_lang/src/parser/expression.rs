use crate::ast::Expression;
use crate::error::Error;
use crate::parser::Parser;

impl<'a> Parser<'a> {
    fn parse_expression(&mut self) -> Result<Expression, Error> {
        todo!()
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

        assert_eq!(expression, Expression::Identifier("fooobar".to_string()));
    }
}

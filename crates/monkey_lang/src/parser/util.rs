use crate::error::Error;
use crate::parser::Parser;
use crate::token::{Token, TokenType};

impl<'a> Parser<'a> {
    pub(crate) fn peek_token(&mut self) -> Option<&Token> {
        self.lexer.peek()
    }

    pub(crate) fn next_token(&mut self) -> Result<Token, Error> {
        if let Some(token) = self.lexer.next() {
            Ok(token)
        } else {
            Err(Error::MissingToken)
        }
    }

    pub(crate) fn expect_next_token(
        &mut self,
        expected_token_variant: TokenType,
    ) -> Result<Token, Error> {
        if let Some(peek_token) = self.peek_token() {
            if peek_token.variant != expected_token_variant {
                Err(Error::UnexpectedToken(peek_token.literal.clone()))
            } else {
                // we want to return the actual token
                self.next_token()
            }
        } else {
            Err(Error::MissingToken)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::error::Error;
    use crate::lexer::Lexer;
    use crate::parser::Parser;
    use crate::token::{Token, TokenType};

    #[test]
    fn expect_next_token() {
        let input = "x = 5";
        let lexer = Lexer::new(input.chars());
        let mut parser = Parser::new(lexer);

        // error condition
        assert_eq!(
            parser.expect_next_token(TokenType::ASSIGN),
            Err(Error::UnexpectedToken("x".to_string()))
        );

        assert_eq!(
            parser.expect_next_token(TokenType::IDENT),
            Ok(Token::new(TokenType::IDENT, "x"))
        );
    }
}
use crate::error::Error;
use crate::parser::Parser;
use crate::token::{Token, TokenType};

impl<'a> Parser<'a> {
    pub(crate) fn peek_token(&mut self) -> Option<&Token> {
        self.lexer.peek()
    }

    pub(crate) fn peek_token_return_err(&mut self) -> Result<&Token, Error> {
        if let Some(peek_token) = self.lexer.peek() {
            Ok(peek_token)
        } else {
            Err(Error::MissingToken)
        }
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

#[derive(PartialEq, PartialOrd)]
pub(crate) enum Precedence {
    LOWEST,
    EQUALS,        // ==
    LESSORGREATER, // > or <
    SUM,           // +
    PRODUCT,       // *
    PREFIX,        // -X or !X
    CALL,          // fn(X)
}

impl Default for Precedence {
    fn default() -> Self {
        Self::LOWEST
    }
}

impl Precedence {
    pub(crate) fn get_precedence(token_type: &TokenType) -> Self {
        match token_type {
            TokenType::EQUAL => Self::EQUALS,
            TokenType::NOTEQUAL => Self::EQUALS,
            TokenType::LESSTHAN => Self::LESSORGREATER,
            TokenType::GREATERTHAN => Self::LESSORGREATER,
            TokenType::PLUS => Self::SUM,
            TokenType::MINUS => Self::SUM,
            TokenType::SLASH => Self::PRODUCT,
            TokenType::ASTERISK => Self::PRODUCT,
            _ => Self::LOWEST,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::error::Error;
    use crate::lexer::Lexer;
    use crate::parser::util::Precedence;
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

    #[test]
    fn precedence_ordering() {
        assert!(Precedence::CALL > Precedence::PREFIX);
        assert!(Precedence::PREFIX > Precedence::PRODUCT);
        assert!(Precedence::PRODUCT > Precedence::SUM);
        assert!(Precedence::SUM > Precedence::LESSORGREATER);
        assert!(Precedence::LESSORGREATER > Precedence::LOWEST);
    }
}

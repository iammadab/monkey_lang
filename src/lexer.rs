use crate::token::{Token, TokenType};

struct Lexer {
    input: String,
}

impl Lexer {
    fn new(input: &str) -> Self {
        Self {
            input: input.to_string(),
        }
    }

    fn next_token(&self) -> Token {
        Token::new(TokenType::ASSIGN, "=")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_token() {
        let input = "=+(){},;";
        let lexer = Lexer::new(input);

        assert_eq!(lexer.next_token(), Token::new(TokenType::ASSIGN, "="));
        assert_eq!(lexer.next_token(), Token::new(TokenType::PLUS, "+"));
        assert_eq!(lexer.next_token(), Token::new(TokenType::LEFTPAREN, "("));
        assert_eq!(lexer.next_token(), Token::new(TokenType::RIGHTPAREN, ")"));
        assert_eq!(lexer.next_token(), Token::new(TokenType::LEFTBRACE, "{"));
        assert_eq!(lexer.next_token(), Token::new(TokenType::RIGHTBRACE, "}"));
        assert_eq!(lexer.next_token(), Token::new(TokenType::COMMA, ","));
        assert_eq!(lexer.next_token(), Token::new(TokenType::SEMICOLON, ";"));
        assert_eq!(lexer.next_token(), Token::new(TokenType::EOF, ""));
    }
}

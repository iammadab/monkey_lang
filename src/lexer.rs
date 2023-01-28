use crate::token::{Token, TokenType};
use std::str::Chars;

const NULL_CHAR: char = '\0';

struct Lexer<'a> {
    input: Chars<'a>,
}

impl<'a> Lexer<'a> {
    fn new(input: Chars<'a>) -> Self {
        Self { input }
    }

    fn read_next_char(&mut self) -> char {
        self.input.next().unwrap_or(NULL_CHAR)
    }

    // TODO: implement proper error handling
    fn next_token(&mut self) -> Token {
        match self.read_next_char() {
            '=' => Token::new(TokenType::ASSIGN, "="),
            ';' => Token::new(TokenType::SEMICOLON, ";"),
            '(' => Token::new(TokenType::LEFTPAREN, "("),
            ')' => Token::new(TokenType::RIGHTPAREN, ")"),
            ',' => Token::new(TokenType::COMMA, ","),
            '+' => Token::new(TokenType::PLUS, "+"),
            '{' => Token::new(TokenType::LEFTBRACE, "{"),
            '}' => Token::new(TokenType::RIGHTBRACE, "}"),
            NULL_CHAR => Token::new(TokenType::EOF, ""),
            actual_char => panic!("unexpected char {}", actual_char),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_token() {
        let input = "=+(){},;";
        let mut lexer = Lexer::new(input.chars());

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

use crate::token::{look_up_ident, Token, TokenType};
use std::iter::Peekable;
use std::str::Chars;

const NULL_CHAR: char = '\0';

struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    fn new(input: Chars<'a>) -> Self {
        Self {
            input: input.peekable(),
        }
    }

    // TODO: implement proper error handling
    fn next_token(&mut self) -> Token {
        self.skip_white_space();

        match self.peek_next_char() {
            '=' => Token::new(TokenType::ASSIGN, &self.read_next_char_as_string()),
            ';' => Token::new(TokenType::SEMICOLON, &self.read_next_char_as_string()),
            '(' => Token::new(TokenType::LEFTPAREN, &self.read_next_char_as_string()),
            ')' => Token::new(TokenType::RIGHTPAREN, &self.read_next_char_as_string()),
            ',' => Token::new(TokenType::COMMA, &self.read_next_char_as_string()),
            '+' => Token::new(TokenType::PLUS, &self.read_next_char_as_string()),
            '{' => Token::new(TokenType::LEFTBRACE, &self.read_next_char_as_string()),
            '}' => Token::new(TokenType::RIGHTBRACE, &self.read_next_char_as_string()),

            &NULL_CHAR => Token::new(TokenType::EOF, ""),

            // if we don't match any above, we should check if it's a letter
            char_value => {
                // TODO: implement special is alphabetic function
                if char_value.is_alphabetic() {
                    // read an identifier and return
                    let identifier = self.read_identifier();
                    let identifier_token_type = look_up_ident(&identifier);
                    Token::new(identifier_token_type, &identifier)
                } else if char_value.is_numeric() {
                    let number = self.read_number();
                    Token::new(TokenType::INT, &number)
                } else {
                    Token::new(TokenType::ILLEGAL, &char_value.to_string())
                }
            }
        }
    }

    fn read_next_char_as_string(&mut self) -> String {
        self.read_next_char().to_string()
    }

    fn read_next_char(&mut self) -> char {
        self.input.next().unwrap_or(NULL_CHAR)
    }

    fn peek_next_char(&mut self) -> &char {
        self.input.peek().unwrap_or(&NULL_CHAR)
    }

    fn skip_white_space(&mut self) {
        while self.peek_next_char().is_whitespace() {
            self.read_next_char();
        }
    }

    fn read_identifier(&mut self) -> String {
        // keep reading chars as long as they are alphabetic
        let mut identifier = String::new();
        while self.peek_next_char().is_alphabetic() {
            identifier.push(self.read_next_char())
        }
        identifier
    }

    fn read_number(&mut self) -> String {
        let mut number = String::new();
        while self.peek_next_char().is_numeric() {
            number.push(self.read_next_char())
        }
        number
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_token_simple_case() {
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

    #[test]
    fn next_token_medium_case() {
        // this will use strings that look like monkey_lang
        // will make use of identifiers too
        let input = "let five = 5;\
            let ten = 10;\
            let add = fn(x, y) {\
                x + y\
            };\
            let result = add(five, ten);";
        let mut lexer = Lexer::new(input.chars());

        assert_eq!(lexer.next_token(), Token::new(TokenType::LET, "let"));
        assert_eq!(lexer.next_token(), Token::new(TokenType::IDENT, "five"));
        assert_eq!(lexer.next_token(), Token::new(TokenType::ASSIGN, "="));
        assert_eq!(lexer.next_token(), Token::new(TokenType::INT, "5"));
    }
}

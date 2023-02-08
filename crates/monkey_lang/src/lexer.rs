use crate::token::{look_up_ident, Token, TokenType};
use std::iter::Peekable;
use std::str::Chars;

const NULL_CHAR: char = '\0';

// TODO: Implement an iterator for the lexer
//  should stop once we reach the eof
pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}

impl<'a> Lexer<'a> {
    pub fn new(input: Chars<'a>) -> Self {
        Self {
            input: input.peekable(),
        }
    }

    // TODO: maybe return a Result<Option<Token>> here
    //  were Err means Illegal and None means EOF
    pub fn next_token(&mut self) -> Option<Token> {
        self.skip_white_space();

        match self.peek_next_char() {
            '=' => {
                self.build_new_token_optional_double_char(TokenType::ASSIGN, &'=', TokenType::EQUAL)
            }
            ';' => self.build_new_token(TokenType::SEMICOLON),
            '(' => self.build_new_token(TokenType::LEFTPAREN),
            ')' => self.build_new_token(TokenType::RIGHTPAREN),
            ',' => self.build_new_token(TokenType::COMMA),
            '+' => self.build_new_token(TokenType::PLUS),
            '-' => self.build_new_token(TokenType::MINUS),
            '!' => self.build_new_token_optional_double_char(
                TokenType::BANG,
                &'=',
                TokenType::NOTEQUAL,
            ),
            '*' => self.build_new_token(TokenType::ASTERISK),
            '/' => self.build_new_token(TokenType::SLASH),
            '<' => self.build_new_token(TokenType::LESSTHAN),
            '>' => self.build_new_token(TokenType::GREATERTHAN),
            '{' => self.build_new_token(TokenType::LEFTBRACE),
            '}' => self.build_new_token(TokenType::RIGHTBRACE),

            &NULL_CHAR => None,

            // if we don't match any above, we should check if it's a letter
            char_value => {
                // TODO: implement special is alphabetic function
                if char_value.is_alphabetic() {
                    // read an identifier and return
                    let identifier = self.read_identifier();
                    let identifier_token_type = look_up_ident(&identifier);
                    Lexer::build_new_token_with_literal(identifier_token_type, &identifier)
                } else if char_value.is_numeric() {
                    let number = self.read_number();
                    Lexer::build_new_token_with_literal(TokenType::INT, &number)
                } else {
                    Lexer::build_new_token_with_literal(TokenType::ILLEGAL, &char_value.to_string())
                }
            }
        }
    }

    fn read_identifier(&mut self) -> String {
        self.read_while(|c| c.is_alphabetic())
    }

    // TODO: we should be able to read non integer numbers also
    fn read_number(&mut self) -> String {
        self.read_while(|c| c.is_numeric())
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

    fn read_while<T>(&mut self, predicate: T) -> String
    where
        T: Fn(&char) -> bool,
    {
        let mut result = String::new();
        while predicate(self.peek_next_char()) {
            result.push(self.read_next_char())
        }
        result
    }

    fn build_new_token(&mut self, token_type: TokenType) -> Option<Token> {
        Some(Token::new(token_type, &self.read_next_char_as_string()))
    }

    fn build_new_token_with_literal(token_type: TokenType, literal: &str) -> Option<Token> {
        Some(Token::new(token_type, literal))
    }

    fn build_new_token_optional_double_char(
        &mut self,
        single_match_token_type: TokenType,
        expected_next_char: &char,
        double_match_token_type: TokenType,
    ) -> Option<Token> {
        let mut matches = self.read_next_char_as_string();
        if self.peek_next_char() == expected_next_char {
            matches.push_str(&self.read_next_char_as_string());
            Lexer::build_new_token_with_literal(double_match_token_type, &matches)
        } else {
            Lexer::build_new_token_with_literal(single_match_token_type, &matches)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_token_simple_case() {
        let input = "=+(){},;";
        let mut lexer = Lexer::new(input.chars());

        assert_eq!(lexer.next_token(), Some(Token::new(TokenType::ASSIGN, "=")));
        assert_eq!(lexer.next_token(), Some(Token::new(TokenType::PLUS, "+")));
        assert_eq!(
            lexer.next_token(),
            Some(Token::new(TokenType::LEFTPAREN, "("))
        );
        assert_eq!(
            lexer.next_token(),
            Some(Token::new(TokenType::RIGHTPAREN, ")"))
        );
        assert_eq!(
            lexer.next_token(),
            Some(Token::new(TokenType::LEFTBRACE, "{"))
        );
        assert_eq!(
            lexer.next_token(),
            Some(Token::new(TokenType::RIGHTBRACE, "}"))
        );
        assert_eq!(lexer.next_token(), Some(Token::new(TokenType::COMMA, ",")));
        assert_eq!(
            lexer.next_token(),
            Some(Token::new(TokenType::SEMICOLON, ";"))
        );
        assert_eq!(lexer.next_token(), None);
    }

    #[test]
    fn next_token_complex_case() {
        // this will use strings that look like monkey_lang
        // will make use of identifiers too
        let input = "let five = 5;\
            let ten = 10;\
            let add = fn(x, y) {\
                x + y;\
            };\
            \
            let result = add(five, ten);\
            !-/*5;\
            5 < 10 > 5;\
            \
            if (5 < 10) {\
                return true;\
             } else {\
                return false;\
            }\
            \
            10 == 10;\
            10 != 9;";

        let mut lexer = Lexer::new(input.chars());

        assert_eq!(lexer.next_token(), Some(Token::new(TokenType::LET, "let")));
        assert_eq!(
            lexer.next_token(),
            Some(Token::new(TokenType::IDENT, "five"))
        );
        assert_eq!(lexer.next_token(), Some(Token::new(TokenType::ASSIGN, "=")));
        assert_eq!(lexer.next_token(), Some(Token::new(TokenType::INT, "5")));
        assert_eq!(
            lexer.next_token(),
            Some(Token::new(TokenType::SEMICOLON, ";"))
        );
        assert_eq!(lexer.next_token(), Some(Token::new(TokenType::LET, "let")));
        assert_eq!(
            lexer.next_token(),
            Some(Token::new(TokenType::IDENT, "ten"))
        );
        assert_eq!(lexer.next_token(), Some(Token::new(TokenType::ASSIGN, "=")));
        assert_eq!(lexer.next_token(), Some(Token::new(TokenType::INT, "10")));
        assert_eq!(
            lexer.next_token(),
            Some(Token::new(TokenType::SEMICOLON, ";"))
        );
        assert_eq!(lexer.next_token(), Some(Token::new(TokenType::LET, "let")));
        assert_eq!(
            lexer.next_token(),
            Some(Token::new(TokenType::IDENT, "add"))
        );
        assert_eq!(lexer.next_token(), Some(Token::new(TokenType::ASSIGN, "=")));
        assert_eq!(
            lexer.next_token(),
            Some(Token::new(TokenType::FUNCTION, "fn"))
        );
        assert_eq!(
            lexer.next_token(),
            Some(Token::new(TokenType::LEFTPAREN, "("))
        );
        assert_eq!(lexer.next_token(), Some(Token::new(TokenType::IDENT, "x")));
        assert_eq!(lexer.next_token(), Some(Token::new(TokenType::COMMA, ",")));
        assert_eq!(lexer.next_token(), Some(Token::new(TokenType::IDENT, "y")));
        assert_eq!(
            lexer.next_token(),
            Some(Token::new(TokenType::RIGHTPAREN, ")"))
        );
        assert_eq!(
            lexer.next_token(),
            Some(Token::new(TokenType::LEFTBRACE, "{"))
        );
        assert_eq!(lexer.next_token(), Some(Token::new(TokenType::IDENT, "x")));
        assert_eq!(lexer.next_token(), Some(Token::new(TokenType::PLUS, "+")));
        assert_eq!(lexer.next_token(), Some(Token::new(TokenType::IDENT, "y")));
        assert_eq!(
            lexer.next_token(),
            Some(Token::new(TokenType::SEMICOLON, ";"))
        );
        assert_eq!(
            lexer.next_token(),
            Some(Token::new(TokenType::RIGHTBRACE, "}"))
        );
        assert_eq!(
            lexer.next_token(),
            Some(Token::new(TokenType::SEMICOLON, ";"))
        );
        assert_eq!(lexer.next_token(), Some(Token::new(TokenType::LET, "let")));
        assert_eq!(
            lexer.next_token(),
            Some(Token::new(TokenType::IDENT, "result"))
        );
        assert_eq!(lexer.next_token(), Some(Token::new(TokenType::ASSIGN, "=")));
        assert_eq!(
            lexer.next_token(),
            Some(Token::new(TokenType::IDENT, "add"))
        );
        assert_eq!(
            lexer.next_token(),
            Some(Token::new(TokenType::LEFTPAREN, "("))
        );
        assert_eq!(
            lexer.next_token(),
            Some(Token::new(TokenType::IDENT, "five"))
        );
        assert_eq!(lexer.next_token(), Some(Token::new(TokenType::COMMA, ",")));
        assert_eq!(
            lexer.next_token(),
            Some(Token::new(TokenType::IDENT, "ten"))
        );
        assert_eq!(
            lexer.next_token(),
            Some(Token::new(TokenType::RIGHTPAREN, ")"))
        );
        assert_eq!(
            lexer.next_token(),
            Some(Token::new(TokenType::SEMICOLON, ";"))
        );
        assert_eq!(lexer.next_token(), Some(Token::new(TokenType::BANG, "!")));
        assert_eq!(lexer.next_token(), Some(Token::new(TokenType::MINUS, "-")));
        assert_eq!(lexer.next_token(), Some(Token::new(TokenType::SLASH, "/")));
        assert_eq!(
            lexer.next_token(),
            Some(Token::new(TokenType::ASTERISK, "*"))
        );
        assert_eq!(lexer.next_token(), Some(Token::new(TokenType::INT, "5")));
        assert_eq!(
            lexer.next_token(),
            Some(Token::new(TokenType::SEMICOLON, ";"))
        );
        assert_eq!(lexer.next_token(), Some(Token::new(TokenType::INT, "5")));
        assert_eq!(
            lexer.next_token(),
            Some(Token::new(TokenType::LESSTHAN, "<"))
        );
        assert_eq!(lexer.next_token(), Some(Token::new(TokenType::INT, "10")));
        assert_eq!(
            lexer.next_token(),
            Some(Token::new(TokenType::GREATERTHAN, ">"))
        );
        assert_eq!(lexer.next_token(), Some(Token::new(TokenType::INT, "5")));
        assert_eq!(
            lexer.next_token(),
            Some(Token::new(TokenType::SEMICOLON, ";"))
        );
        assert_eq!(lexer.next_token(), Some(Token::new(TokenType::IF, "if")));
        assert_eq!(
            lexer.next_token(),
            Some(Token::new(TokenType::LEFTPAREN, "("))
        );
        assert_eq!(lexer.next_token(), Some(Token::new(TokenType::INT, "5")));
        assert_eq!(
            lexer.next_token(),
            Some(Token::new(TokenType::LESSTHAN, "<"))
        );
        assert_eq!(lexer.next_token(), Some(Token::new(TokenType::INT, "10")));
        assert_eq!(
            lexer.next_token(),
            Some(Token::new(TokenType::RIGHTPAREN, ")"))
        );
        assert_eq!(
            lexer.next_token(),
            Some(Token::new(TokenType::LEFTBRACE, "{"))
        );
        assert_eq!(
            lexer.next_token(),
            Some(Token::new(TokenType::RETURN, "return"))
        );
        assert_eq!(
            lexer.next_token(),
            Some(Token::new(TokenType::TRUE, "true"))
        );
        assert_eq!(
            lexer.next_token(),
            Some(Token::new(TokenType::SEMICOLON, ";"))
        );
        assert_eq!(
            lexer.next_token(),
            Some(Token::new(TokenType::RIGHTBRACE, "}"))
        );
        assert_eq!(
            lexer.next_token(),
            Some(Token::new(TokenType::ELSE, "else"))
        );
        assert_eq!(
            lexer.next_token(),
            Some(Token::new(TokenType::LEFTBRACE, "{"))
        );
        assert_eq!(
            lexer.next_token(),
            Some(Token::new(TokenType::RETURN, "return"))
        );
        assert_eq!(
            lexer.next_token(),
            Some(Token::new(TokenType::FALSE, "false"))
        );
        assert_eq!(
            lexer.next_token(),
            Some(Token::new(TokenType::SEMICOLON, ";"))
        );
        assert_eq!(
            lexer.next_token(),
            Some(Token::new(TokenType::RIGHTBRACE, "}"))
        );
        assert_eq!(lexer.next_token(), Some(Token::new(TokenType::INT, "10")));
        assert_eq!(lexer.next_token(), Some(Token::new(TokenType::EQUAL, "==")));
        assert_eq!(lexer.next_token(), Some(Token::new(TokenType::INT, "10")));
        assert_eq!(
            lexer.next_token(),
            Some(Token::new(TokenType::SEMICOLON, ";"))
        );
        assert_eq!(lexer.next_token(), Some(Token::new(TokenType::INT, "10")));
        assert_eq!(
            lexer.next_token(),
            Some(Token::new(TokenType::NOTEQUAL, "!="))
        );
        assert_eq!(lexer.next_token(), Some(Token::new(TokenType::INT, "9")));
        assert_eq!(
            lexer.next_token(),
            Some(Token::new(TokenType::SEMICOLON, ";"))
        );
        assert_eq!(lexer.next_token(), None);
    }

    #[test]
    fn lexer_as_iterator() {
        let input = "=+(){},;";
        let mut lexer = Lexer::new(input.chars());
        let mut lexer = lexer.into_iter();

        assert_eq!(lexer.next(), Some(Token::new(TokenType::ASSIGN, "=")));
        assert_eq!(lexer.next(), Some(Token::new(TokenType::PLUS, "+")));
        assert_eq!(lexer.next(), Some(Token::new(TokenType::LEFTPAREN, "(")));
        assert_eq!(lexer.next(), Some(Token::new(TokenType::RIGHTPAREN, ")")));
        assert_eq!(lexer.next(), Some(Token::new(TokenType::LEFTBRACE, "{")));
        assert_eq!(lexer.next(), Some(Token::new(TokenType::RIGHTBRACE, "}")));
        assert_eq!(lexer.next(), Some(Token::new(TokenType::COMMA, ",")));
        assert_eq!(lexer.next(), Some(Token::new(TokenType::SEMICOLON, ";")));
        assert_eq!(lexer.next(), None);
        assert_eq!(lexer.next(), None);
    }
}

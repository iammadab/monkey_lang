use crate::token::{Token, TokenType};

// TODO: refactor lexer to use an iterator
struct Lexer {
    input: Vec<char>,
    curr_position: usize,
    next_position: usize,
    curr_char: char,
}

impl Lexer {
    fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            curr_position: 0,
            next_position: 0,
            // TODO: factor out the null char
            curr_char: '\0',
        }
    }

    // TODO: could we use an iterator instead??
    fn read_next_char(&mut self) {
        if self.next_position >= self.input.len() {
            self.curr_char = '\0';
        } else {
            self.curr_char = self.input[self.next_position];
        }
        self.curr_position = self.next_position;
        self.next_position += 1;
    }

    fn next_token(&mut self) -> Token {
        self.read_next_char();
        match self.curr_char {
            '=' => Token::new(TokenType::ASSIGN, "="),
            ';' => Token::new(TokenType::SEMICOLON, ";"),
            '(' => Token::new(TokenType::LEFTPAREN, "("),
            ')' => Token::new(TokenType::RIGHTPAREN, ")"),
            ',' => Token::new(TokenType::COMMA, ","),
            '+' => Token::new(TokenType::PLUS, "+"),
            '{' => Token::new(TokenType::LEFTBRACE, "{"),
            '}' => Token::new(TokenType::RIGHTBRACE, "}"),
            '\0' => Token::new(TokenType::EOF, ""),
            // TODO: do proper error handling
            _ => panic!("unexpected char {}", self.curr_char),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_token() {
        let input = "=+(){},;";
        let mut lexer = Lexer::new(input);

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

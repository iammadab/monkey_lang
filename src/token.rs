use std::collections::HashMap;

#[derive(PartialEq, Debug)]
pub(crate) struct Token {
    variant: TokenType,
    literal: String,
}

impl Token {
    pub(crate) fn new(variant: TokenType, literal: &str) -> Self {
        Self {
            variant,
            literal: literal.to_string(),
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub(crate) enum TokenType {
    ILLEGAL,
    EOF,

    // Identifiers + literals
    IDENT,
    INT,

    // Operators
    ASSIGN,
    PLUS,
    MINUS,
    BANG,
    ASTERISK,
    SLASH,

    LESSTHAN,
    GREATERTHAN,
    EQUAL,
    NOTEQUAL,

    // Delimiters
    COMMA,
    SEMICOLON,

    LEFTPAREN,
    RIGHTPAREN,
    LEFTBRACE,
    RIGHTBRACE,

    // Keywords
    FUNCTION,
    LET,
    TRUE,
    FALSE,
    IF,
    ELSE,
    RETURN,
}

pub(crate) fn look_up_ident(ident: &str) -> TokenType {
    let keyword_map: HashMap<&str, TokenType> = HashMap::from([
        ("fn", TokenType::FUNCTION),
        ("let", TokenType::LET),
        ("true", TokenType::TRUE),
        ("false", TokenType::FALSE),
        ("if", TokenType::IF),
        ("else", TokenType::ELSE),
        ("return", TokenType::RETURN),
    ]);

    keyword_map.get(ident).cloned().unwrap_or(TokenType::IDENT)
}

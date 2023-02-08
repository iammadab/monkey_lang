use std::collections::HashMap;

#[derive(PartialEq, Debug, Clone)]
pub struct Token {
    pub variant: TokenType,
    pub(crate) literal: String,
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
pub enum TokenType {
    // TODO: potentially get rid of the illegal variant
    //  actually we should get rid of this, if the lexer
    //  hits this while tokenizing, we should return an error
    //  that gets propagated up
    ILLEGAL,

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

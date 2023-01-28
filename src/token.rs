#[derive(PartialEq, Debug)]
pub(crate) enum TokenType {
    ILLEGAL,
    EOF,

    // Identifiers + literals
    IDENT,
    INT,

    // Operators
    ASSIGN,
    PLUS,

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
}

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

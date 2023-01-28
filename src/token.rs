enum TokenType {
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

struct Token {
    variant: TokenType,
    // TODO: this should be optional, not all variants need a literal
    literal: String,
}

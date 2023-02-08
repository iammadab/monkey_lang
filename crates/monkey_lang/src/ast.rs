use crate::token::Token;

/// Enum representing the different type of statements we handle
#[derive(Debug, PartialEq)]
pub(crate) enum Statement {
    /// Represents let statements of the form
    /// let <identifier> = <expression>;
    /// e.g let a = 2;
    Let { name: String, value: Expression },
    /// Represents statements of the form
    /// return <expression>;
    /// e.g return 2 + 2;
    Return { return_value: Expression },
}

/// Enum representing the different type of expressions we handle
#[derive(Debug, PartialEq)]
pub(crate) enum Expression {
    /// Represents the name of something
    Identifier(String),
}

/// Represents the program as a series of statements
pub(crate) struct Program {
    pub(crate) statements: Vec<Statement>,
}

impl Program {
    pub(crate) fn new() -> Self {
        Self {
            statements: Vec::new(),
        }
    }
}

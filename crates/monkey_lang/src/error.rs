use crate::ast::InfixOperator;
use crate::object::Object;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum Error {
    // Parsing errors
    #[error("unexpected token: {0}")]
    UnexpectedToken(String),
    #[error("expected token, found none")]
    MissingToken,
    #[error("failed to convert {0} to i64 value")]
    InvalidIntegerValue(String),
    #[error("failed to convert {0} to boolean value")]
    InvalidBooleanValue(String),
    #[error("failed to convert {0} to prefix operator")]
    InvalidPrefixOperator(String),
    #[error("failed to convert {0} to infix operator")]
    InvalidInfixOperator(String),

    // Evaluation errors
    #[error("type mismatch: {left} {operator} {right}")]
    TypeMismatch {
        left: String,
        operator: String,
        right: String,
    },
    #[error("unknown operator: {0}")]
    UnknownOperator(String),
    #[error("identifier not found: {0}")]
    IdentifierNotFound(String),
}

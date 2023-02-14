use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum Error {
    // TODO: add the expected token to this
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
}

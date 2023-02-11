use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub(crate) enum Error {
    #[error("unexpected token: {0}")]
    UnexpectedToken(String),
    #[error("expected token, found none")]
    MissingToken,
    #[error("failed to convert {0} to i64 value")]
    InvalidIntValue(String),
}

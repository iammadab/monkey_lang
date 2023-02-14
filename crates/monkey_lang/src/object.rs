use std::fmt::{Display, Formatter};
use thiserror::__private::DisplayAsDisplay;

/// Enum to describe the internal object system for our values
#[derive(PartialEq, Debug)]
pub(crate) enum Object {
    Integer(i64),
    Boolean(bool),
    Null,
}

impl Display for Object {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Integer(val) => f.write_str(&format!("{}", val)),
            Self::Boolean(val) => f.write_str(&format!("{}", val)),
            Self::Null => f.write_str("null"),
        }
    }
}

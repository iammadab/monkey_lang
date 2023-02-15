use std::fmt::{Display, Formatter};

/// Enum to describe the internal object system for our values
#[derive(PartialEq, Debug, Clone)]
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

impl Object {
    /// Returns the bool velue of an object
    /// Null, false and 0 objects are considered false
    /// everything else is considered true
    pub(crate) fn is_truthy(&self) -> bool {
        match self {
            Object::Boolean(val) => val.to_owned(),
            Object::Null => false,
            Object::Integer(val) => val != &0,
        }
    }
}

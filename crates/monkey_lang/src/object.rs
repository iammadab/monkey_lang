use std::fmt::{Display, Formatter};

/// Struct to hold result of evaluations
pub(crate) struct EvaluationValue {
    pub(crate) object: Object,
    pub(crate) is_return_value: bool,
}

impl Default for EvaluationValue {
    fn default() -> Self {
        Self {
            object: Object::Null,
            is_return_value: false,
        }
    }
}

impl From<Object> for EvaluationValue {
    fn from(value: Object) -> Self {
        Self {
            object: value,
            is_return_value: false,
        }
    }
}

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

    pub(crate) fn to_type_string(&self) -> String {
        match self {
            Object::Boolean(_) => "BOOLEAN".to_string(),
            Object::Integer(_) => "INTEGER".to_string(),
            Object::Null => "NULL".to_string(),
        }
    }
}

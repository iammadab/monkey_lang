/// Enum to describe the internal object system for our values
#[derive(PartialEq, Debug)]
pub(crate) enum Object {
    Integer(i64),
    Boolean(bool),
    Null,
}

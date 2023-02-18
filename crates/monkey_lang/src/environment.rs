use crate::error::Error;
use crate::object::Object;
use std::collections::HashMap;

// TODO: is this documentation up to date?
/// Hashmap to keep track of identifiers and function definitions
pub struct Environment {
    store: HashMap<String, Object>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }

    pub(crate) fn get(&self, name: String) -> Result<Object, Error> {
        self.store
            .get(name.as_str())
            .cloned()
            .ok_or(Error::IdentifierNotFound(name))
    }

    pub(crate) fn set(&mut self, name: String, obj: Object) -> Option<Object> {
        self.store.insert(name, obj)
    }
}

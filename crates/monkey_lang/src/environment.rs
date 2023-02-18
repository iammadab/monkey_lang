use std::collections::HashMap;
use crate::object::Object;

// TODO: is this documentation up to date?
/// Hashmap to keep track of identifiers and function definitions
pub(crate) struct Environment {
   store: HashMap<String, Object>
}

impl Environment {
    pub(crate) fn new() -> Self {
        Self {
            store: HashMap::new()
        }
    }

    pub(crate) fn get(&self, name: String) -> Option<&Object> {
        self.store.get(name.as_str())
    }

    pub(crate) fn set(&mut self, name: String, obj: Object) -> Option<Object> {
        self.store.insert(name, obj)
    }
}
use crate::value::Value;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Default)]
pub(crate) struct Environment {
    bindings: HashMap<String, Value>,
}

impl Environment {
    pub(crate) fn store_binding(&mut self, name: String, value: Value) {
        self.bindings.insert(name, value);
    }
}

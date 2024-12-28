use crate::{environment::Environment, utils, value::Value};

#[derive(Debug, PartialEq)]
pub struct BindingUsage {
    pub name: String,
}

impl BindingUsage {
    pub fn new(s: &str) -> Result<(&str, Self), String> {
        let (s, name) = utils::extract_ident(s)?;

        Ok((
            s,
            Self {
                name: name.to_string(),
            },
        ))
    }

    pub(crate) fn evaluate(&self, env: &Environment) -> Result<Value, String> {
        env.get_binding_value(&self.name)
    }
}

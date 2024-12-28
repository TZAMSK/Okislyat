use crate::{
    environment::{self, Environment},
    expression::Expression,
    utils,
};

#[derive(Debug, PartialEq)]
pub struct BindingDef {
    pub name: String,
    pub value: Expression,
}

impl BindingDef {
    pub fn new(s: &str) -> Result<(&str, Self), String> {
        let s = utils::tag("let", s)?;
        let (s, _) = utils::extract_whitespace1(s)?;

        let (s, name) = utils::extract_ident(s)?;
        let (s, _) = utils::extract_whitespace(s);

        let s = utils::tag("=", s)?;
        let (s, _) = utils::extract_whitespace(s);

        let (s, value) = Expression::new(s)?;

        Ok((
            s,
            Self {
                name: name.to_string(),
                value,
            },
        ))
    }

    pub(crate) fn evaluate(&self, environment: &mut Environment) {
        environment.store_binding(self.name.clone(), self.value.evaluate());
    }
}

use crate::{number::Number, operations::Operation, utils};

#[derive(Debug, PartialEq)]
pub struct Expression {
    pub lhs: Number,
    pub rhs: Number,
    pub op: Operation,
}

impl Expression {
    pub fn new(s: &str) -> (&str, Self) {
        let (s, lhs) = Number::new(s);
        let (s, _) = utils::extract_whitespace(s);

        let (s, op) = Operation::new(s);
        let (s, _) = utils::extract_whitespace(s);

        let (s, rhs) = Number::new(s);

        (s, Self { lhs, rhs, op })
    }
}

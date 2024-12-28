use crate::{number::Number, operations::Operation, utils, value::Value};

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

    pub(crate) fn evaluate(&self) -> Value {
        let Number(lhs) = self.lhs;
        let Number(rhs) = self.rhs;

        let result = match self.op {
            Operation::Add => lhs + rhs,
            Operation::Sub => lhs - rhs,
            Operation::Mul => lhs * rhs,
            Operation::Div => lhs / rhs,
        };

        Value::Number(result)
    }
}

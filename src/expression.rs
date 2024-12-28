use crate::{number::Number, operations::Operation, utils, value::Value};

#[derive(Debug, PartialEq)]
pub enum Expression {
    Number(Number),
    Operation {
        lhs: Number,
        rhs: Number,
        op: Operation,
    },
}

impl Expression {
    pub fn new(s: &str) -> Result<(&str, Self), String> {
        Self::new_operations(s).or_else(|_| Self::new_number(s))
    }

    fn new_operations(s: &str) -> Result<(&str, Self), String> {
        let (s, lhs) = Number::new(s)?;
        let (s, _) = utils::extract_whitespace(s);

        let (s, op) = Operation::new(s)?;
        let (s, _) = utils::extract_whitespace(s);

        let (s, rhs) = Number::new(s)?;

        Ok((s, Self::Operation { lhs, rhs, op }))
    }

    fn new_number(s: &str) -> Result<(&str, Self), String> {
        Number::new(s).map(|(s, number)| (s, Self::Number(number)))
    }

    pub(crate) fn evaluate(&self) -> Value {
        match self {
            Self::Number(Number(n)) => Value::Number(*n),
            Self::Operation { lhs, rhs, op } => {
                let Number(lhs) = lhs;
                let Number(rhs) = rhs;

                let result = match op {
                    Operation::Add => lhs + rhs,
                    Operation::Sub => lhs - rhs,
                    Operation::Mul => lhs * rhs,
                    Operation::Div => lhs / rhs,
                };
                Value::Number(result)
            }
        }
    }
}

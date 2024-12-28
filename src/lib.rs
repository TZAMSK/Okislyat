mod binding_def;
mod environment;
mod expression;
mod number;
mod operations;
mod utils;
mod value;

#[cfg(test)]
mod tests {

    use super::{
        binding_def::BindingDef,
        expression::Expression,
        number::Number,
        operations::Operation,
        utils::{extract_digits, extract_ident, extract_op, extract_whitespace, tag},
        value::Value,
    };

    #[test]
    fn parse_number() {
        assert_eq!(Number::new("123"), ("", Number(123)));
    }

    #[test]
    fn parse_add_op() {
        assert_eq!(Operation::new("+"), ("", Operation::Add));
    }

    #[test]
    fn parse_sub_op() {
        assert_eq!(Operation::new("-"), ("", Operation::Sub));
    }

    #[test]
    fn parse_mul_op() {
        assert_eq!(Operation::new("*"), ("", Operation::Mul));
    }

    #[test]
    fn parse_div_op() {
        assert_eq!(Operation::new("/"), ("", Operation::Div));
    }

    #[test]
    fn parse_one_plus_two() {
        assert_eq!(
            Expression::new("1 + 2"),
            (
                "",
                Expression {
                    lhs: Number(1),
                    rhs: Number(2),
                    op: Operation::Add,
                },
            ),
        );
    }

    #[test]
    fn extract_one_digit() {
        assert_eq!(extract_digits("1+2"), ("+2", "1"));
    }

    #[test]
    fn extract_multiple_digit() {
        assert_eq!(extract_digits("10-20"), ("-20", "10"));
    }

    #[test]
    fn do_not_extract_anything_from_empty_input() {
        assert_eq!(extract_digits(""), ("", ""));
    }

    #[test]
    fn extract_digits_with_no_remainder() {
        assert_eq!(extract_digits("100"), ("", "100"));
    }

    #[test]
    fn extract_star() {
        assert_eq!(extract_op("*3"), ("3", "*"));
    }

    #[test]
    fn extract_spaces() {
        assert_eq!(extract_whitespace("   1"), ("1", "   "));
    }

    #[test]
    fn parse_binding_def() {
        assert_eq!(
            BindingDef::new("let a = 10 / 2"),
            (
                "",
                BindingDef {
                    name: "a".to_string(),
                    value: Expression {
                        lhs: Number(10),
                        rhs: Number(2),
                        op: Operation::Div,
                    },
                },
            ),
        );
    }

    #[test]
    fn extract_alphabetical_ident() {
        assert_eq!(extract_ident("abcdEFG stop"), (" stop", "abcdEFG"));
    }

    #[test]
    fn extract_alphanumerical_ident() {
        assert_eq!(extract_ident("footbar1()"), ("()", "footbar1"));
    }

    #[test]
    fn cannot_extract_ident_beginning_with_number() {
        assert_eq!(extract_ident("123abc"), ("123abc", ""));
    }

    #[test]
    fn tag_word() {
        assert_eq!(tag("let", "let a"), " a");
    }

    #[test]
    fn evaluate_add() {
        assert_eq!(
            Expression {
                lhs: Number(10),
                rhs: Number(10),
                op: Operation::Add,
            }
            .evaluate(),
            Value::Number(20),
        );
    }

    #[test]
    fn evaluate_sub() {
        assert_eq!(
            Expression {
                lhs: Number(1),
                rhs: Number(5),
                op: Operation::Sub,
            }
            .evaluate(),
            Value::Number(-4),
        );
    }

    #[test]
    fn evaluate_mul() {
        assert_eq!(
            Expression {
                lhs: Number(5),
                rhs: Number(6),
                op: Operation::Mul,
            }
            .evaluate(),
            Value::Number(30),
        );
    }

    #[test]
    fn evaluate_div() {
        assert_eq!(
            Expression {
                lhs: Number(200),
                rhs: Number(20),
                op: Operation::Div,
            }
            .evaluate(),
            Value::Number(10),
        );
    }
}

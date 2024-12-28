mod binding_def;
mod binding_usage;
mod environment;
mod expression;
mod number;
mod operations;
mod utils;
mod value;

#[cfg(test)]
mod tests {

    use crate::environment::Environment;

    use super::{
        binding_def::BindingDef,
        binding_usage::BindingUsage,
        expression::Expression,
        number::Number,
        operations::Operation,
        utils::{extract_digits, extract_ident, extract_whitespace, extract_whitespace1, tag},
        value::Value,
    };

    #[test]
    fn parse_number() {
        assert_eq!(Number::new("123"), Ok(("", Number(123))));
    }

    #[test]
    fn parse_add_op() {
        assert_eq!(Operation::new("+"), Ok(("", Operation::Add)));
    }

    #[test]
    fn parse_sub_op() {
        assert_eq!(Operation::new("-"), Ok(("", Operation::Sub)));
    }

    #[test]
    fn parse_mul_op() {
        assert_eq!(Operation::new("*"), Ok(("", Operation::Mul)));
    }

    #[test]
    fn parse_div_op() {
        assert_eq!(Operation::new("/"), Ok(("", Operation::Div)));
    }

    #[test]
    fn parse_one_plus_two() {
        assert_eq!(
            Expression::new("1 + 2"),
            Ok((
                "",
                Expression::Operation {
                    lhs: Number(1),
                    rhs: Number(2),
                    op: Operation::Add,
                },
            )),
        );
    }

    #[test]
    fn extract_one_digit() {
        assert_eq!(extract_digits("1+2"), Ok(("+2", "1")));
    }

    #[test]
    fn extract_multiple_digit() {
        assert_eq!(extract_digits("10-20"), Ok(("-20", "10")));
    }

    #[test]
    fn do_not_extract_digits_when_input_is_invalid() {
        assert_eq!(extract_digits("abcd"), Err("expected digits".to_string()));
    }

    #[test]
    fn extract_digits_with_no_remainder() {
        assert_eq!(extract_digits("100"), Ok(("", "100")));
    }

    #[test]
    fn extract_spaces() {
        assert_eq!(extract_whitespace("   1"), ("1", "   "));
    }

    #[test]
    fn parse_binding_def() {
        assert_eq!(
            BindingDef::new("let a = 10 / 2"),
            Ok((
                "",
                BindingDef {
                    name: "a".to_string(),
                    value: Expression::Operation {
                        lhs: Number(10),
                        rhs: Number(2),
                        op: Operation::Div,
                    },
                },
            )),
        );
    }

    #[test]
    fn extract_alphabetical_ident() {
        assert_eq!(extract_ident("abcdEFG stop"), Ok((" stop", "abcdEFG")));
    }

    #[test]
    fn extract_alphanumerical_ident() {
        assert_eq!(extract_ident("footbar1()"), Ok(("()", "footbar1")));
    }

    #[test]
    fn cannot_extract_ident_beginning_with_number() {
        assert_eq!(
            extract_ident("123abc"),
            Err("expected identifier".to_string()),
        );
    }

    #[test]
    fn tag_word() {
        assert_eq!(tag("let", "let a"), Ok(" a"));
    }

    #[test]
    fn evaluate_add() {
        assert_eq!(
            Expression::Operation {
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
            Expression::Operation {
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
            Expression::Operation {
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
            Expression::Operation {
                lhs: Number(200),
                rhs: Number(20),
                op: Operation::Div,
            }
            .evaluate(),
            Value::Number(10),
        );
    }

    #[test]
    fn parse_number_as_expression() {
        assert_eq!(
            Expression::new("456"),
            Ok(("", Expression::Number(Number(456))))
        );
    }

    #[test]
    fn do_not_extract_spaces1_when_does_not_start_with_them() {
        assert_eq!(
            extract_whitespace1("blah"),
            Err("expected a space".to_string()),
        );
    }

    #[test]
    fn cannot_parse_binding_def_without_space_after_let() {
        assert_eq!(
            BindingDef::new("letaaa=1+2"),
            Err("expected a space".to_string())
        );
    }

    #[test]
    fn parse_binding_usage() {
        assert_eq!(
            BindingUsage::new("abc"),
            Ok((
                "",
                BindingUsage {
                    name: "abc".to_string(),
                }
            ))
        );
    }

    #[test]
    fn eval_existing_binding_usage() {
        let mut env = Environment::default();
        env.store_binding("foo".to_string(), Value::Number(10));

        assert_eq!(
            BindingUsage {
                name: "foo".to_string(),
            }
            .evaluate(&env),
            Ok(Value::Number(10)),
        );
    }

    #[test]
    fn eval_non_existent_binding_usage() {
        let empty_env = Environment::default();

        assert_eq!(
            BindingUsage {
                name: "i_dont_exist".to_string(),
            }
            .evaluate(&empty_env),
            Err("binding with name ‘i_dont_exist’ does not exist".to_string()),
        );
    }
}

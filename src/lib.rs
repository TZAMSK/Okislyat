mod expression;
mod number;
mod operations;
mod utils;

#[cfg(test)]
mod tests {

    use super::{
        expression::Expression,
        number::Number,
        operations::Operation,
        utils::{extract_digits, extract_op},
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
            Expression::new("1+2"),
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
}

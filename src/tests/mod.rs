#[cfg(test)]
mod tests {
    use crate::parser::lexer::*;
    use std::any::Any;

    #[test]
    fn number() {
        let result = lexer(&String::from("1.8e6")).pop().unwrap();
        assert_eq!(
            result,
            TokenValue::Literal(
                LiteralValue::Number(Number::from_values(
                    &Some(String::from("6")),
                    &Some(String::from("8")),
                    &String::from("1")
                )),
                Loc::new(1, 5)
            )
        );
    }

    #[test]
    fn assignment() {
        let output = lexer(&String::from("x=12.3e8"));
        let expected = vec![
            TokenValue::Identifier(
                String::from("x"),
                Loc {
                    line_num: 1,
                    column_num: 2,
                },
            ),
            TokenValue::Operator(
                Operator::Equals,
                Loc {
                    line_num: 1,
                    column_num: 3,
                },
            ),
            TokenValue::Literal(
                LiteralValue::Number(Number {
                    exponent: Some(String::from("8")),
                    decimal: Some(String::from("3")),
                    base: String::from("12"),
                }),
                Loc {
                    line_num: 1,
                    column_num: 9,
                },
            ),
        ];
        assert_eq!(output, expected)
    }
}

#[cfg(test)]
mod tests {
    use std::any::Any;

    #[test]
    fn number() {
        use crate::parser::*;
        let result = lexer(&String::from("1.8e6")).pop().unwrap();
        assert_eq!(result, TokenValue::Literal(LiteralValue::Number(Number::from_values(&Some(String::from("6")), &Some(String::from("8")), &String::from("1"))), Loc::new(1, 5)));
    }
}

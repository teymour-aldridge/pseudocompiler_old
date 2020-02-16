#[cfg(test)]
mod lexer_tests {
    use crate::parser::lexer::*;
    use std::any::Any;

    #[test]
    fn number() {
        let output = lexer(&String::from("1.03e8"));
        let first_token = &output.get(0).unwrap().token;
        assert!(first_token==&Token::Literal(LiteralValue::Number(Number {
            base: String::from("1"),
            decimal: Some(String::from("03")),
            exponent: Some(String::from("8"))
        })))
    }

    #[test]
    fn assignment() {}

    #[test]
    fn divide() {}

    #[test]
    fn integer_divide() {}

    #[test]
    fn add() {}

    #[test]
    fn multiply() {}

    #[test]
    fn minus() {}

    #[test]
    fn boolean() {}

    #[test]
    fn string() {}

    #[test]
    fn list() {}

    #[test]
    fn dot() {}

    #[test]
    fn for_loop() {}

    #[test]
    fn while_loop() {}

    #[test]
    fn function() {}
}

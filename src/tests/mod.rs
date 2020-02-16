#[cfg(test)]
mod lexer_tests {
    use crate::parser::lexer::*;
    use std::any::Any;

    #[test]
    fn number() {
        let output = lexer(&String::from("1.03e8"));
        let first_token = &output.get(0).unwrap().token;
        assert!(
            first_token
                == &Token::Literal(LiteralValue::Number(Number {
                    base: String::from("1"),
                    decimal: Some(String::from("03")),
                    exponent: Some(String::from("8")),
                }))
        )
    }

    #[test]
    fn assignment() {
        let mut output = lexer(&String::from("x=1"));
        let identifier = output.get(0).unwrap();
        let equals = output.get(1).unwrap();
        let value = output.get(2).unwrap();
        let eos = output.get(3).unwrap();
        assert!(identifier.token == Token::Identifier(String::from("x")));
        assert!(equals.token == Token::Operator(Operator::Equals));
        assert!(
            value.token
                == Token::Literal(LiteralValue::Number(Number {
                    base: String::from("1"),
                    decimal: None,
                    exponent: None,
                }))
        );
        assert!(eos.token == Token::EndOfSequence);
    }

    #[test]
    fn divide() {
        let mut output = lexer(&String::from("x/y"));
        assert!(output.get(0).unwrap().token == Token::Identifier(String::from("x")));
        assert!(output.get(1).unwrap().token == Token::Operator(Operator::Divide));
        assert!(output.get(2).unwrap().token == Token::Identifier(String::from("y")));
    }

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

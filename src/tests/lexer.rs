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
fn double_equals() {
    let mut output = lexer(&String::from("x==1"));
    let identifier = output.get(0).unwrap();
    let equals = output.get(1).unwrap();
    let value = output.get(2).unwrap();
    let eos = output.get(3).unwrap();
    assert!(identifier.token == Token::Identifier(String::from("x")));
    assert!(equals.token == Token::Operator(Operator::DoubleEquals));
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
fn integer_divide() {
    let mut output = lexer(&String::from("x//y"));
    println!("{:?}", output);
    assert!(output.get(0).unwrap().token == Token::Identifier(String::from("x")));
    assert!(output.get(1).unwrap().token == Token::Operator(Operator::IntegerDivide));
    assert!(output.get(2).unwrap().token == Token::Identifier(String::from("y")));
}

#[test]
fn add() {
    let mut output = lexer(&String::from("x+y"));
    assert!(output.get(0).unwrap().token == Token::Identifier(String::from("x")));
    assert!(output.get(1).unwrap().token == Token::Operator(Operator::Plus));
    assert!(output.get(2).unwrap().token == Token::Identifier(String::from("y")));
}

#[test]
fn multiply() {
    let mut output = lexer(&String::from("x*y"));
    assert!(output.get(0).unwrap().token == Token::Identifier(String::from("x")));
    assert!(output.get(1).unwrap().token == Token::Operator(Operator::Times));
    assert!(output.get(2).unwrap().token == Token::Identifier(String::from("y")));
}

#[test]
fn minus() {
    let mut output = lexer(&String::from("x-y"));
    assert!(output.get(0).unwrap().token == Token::Identifier(String::from("x")));
    assert!(output.get(1).unwrap().token == Token::Operator(Operator::Minus));
    assert!(output.get(2).unwrap().token == Token::Identifier(String::from("y")));
}

#[test]
fn and() {
    let mut output = lexer(&String::from("x and y"));
    assert!(output.get(0).unwrap().token == Token::Identifier(String::from("x")));
    assert!(output.get(1).unwrap().token == Token::Operator(Operator::And));
    assert!(output.get(2).unwrap().token == Token::Identifier(String::from("y")));
}

#[test]
fn or() {
    let mut output = lexer(&String::from("x or y"));
    assert!(output.get(0).unwrap().token == Token::Identifier(String::from("x")));
    assert!(output.get(1).unwrap().token == Token::Operator(Operator::Or));
    assert!(output.get(2).unwrap().token == Token::Identifier(String::from("y")));
}

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

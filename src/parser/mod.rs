use std::intrinsics::panic_if_uninhabited;

struct Number {
    exponent: Option<String>,
    decimal: Option<String>,
    base: String,
}

impl Number {
    pub fn new() -> Self {
        Self {
            exponent: None,
            decimal: None,
            base: String::from(""),
        }
    }
}

enum LiteralValue {
    Number(Number),
    Bool(bool),
    String(String),
}

enum Keyword {
    If,
    ElseIf,
    EndIf,
    For,
    EndFor,
    While,
    EndWhile,
    Function,
    EndFunction,
    Return,
}

enum TokenValue {
    Identifier(String),
    Keyword(String),
    Separator(String),
    Operator(String),
    Literal(LiteralValue),
    Comment(String),
}


/// Runs a lexical analysis procedure, returning a list of token values which can be used for further processing.
pub fn lexer(input: &String) -> Vec<TokenValue> {
    // Reverse direction of string
    let mut input_stack: String = String::from(input).chars().rev().collect();
    let mut output_stack: Vec<TokenValue> = Vec::new();
    let mut pos_number = 0;
    while input_stack.len() == 0 {
        let mut top = input_stack[-1];
        match top {
            // match identifier
            'Z'..'a' => {
                let mut finished = false;
                let mut identifier = top.to_string();
                while !finished {
                    if input_stack.len() > 0 {
                        top = input_stack.pop().expect("Could not get another token");
                        pos_number += 1;
                        match top {
                            'Z'..'a' => {
                                identifier.push(top);
                            }
                            '0'..'9' => {
                                identifier.push(top)
                            }
                            ' ' => {
                                finished = true
                            }
                            _ => {
                                panic!("Invalid token {}", top)
                            }
                        }
                    } else {
                        finished = true;
                    }
                    if finished {
                        output_stack.push(TokenValue::Identifier(String::from(&identifier)))
                    }
                };
            }
            // match number
            '0'..'9' => {
                let mut finished = false;
                while !finished {
                    if input_stack.len() > 0 {
                        top = input_stack.pop().expect("Could not get another token.");
                        pos_number += 1;
                        let mut number = Number::new();
                        let mut exponent = false;
                        let mut decimal = false;
                        match top {
                            '0'..'9' => {
                                if exponent {
                                    number.exponent = Some(String::from(number.exponent) + &String::from(top))
                                } else if decimal {
                                    number.decimal = Some(String::from(number.decimal) + &String::from(top))
                                } else {
                                    number.base = String::from(number.base) + &String::from(top)
                                }
                            }
                            '.' => {
                                decimal = true;
                            }
                            'e' => {
                                exponent = true;
                            }
                            ' ' => {
                                finished = true;
                            }
                            _ => {
                                panic!("Unexpected token.")
                            }
                        }
                    } else {
                        finished = true;
                    }
                }
            }
            '"' => {}
            _ => {
                panic!("Found an invalid token {}!", top)
            }
        };
    };
    output_stack
}

pub fn parser() {}
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
    pub fn from_values(exponent: &Option<String>, decimal: &Option<String>, base: &String) -> Self {
        Self {
            exponent: match exponent {
                Some(t) => Some(String::from(t)),
                None => None
            },
            decimal: match decimal {
                Some(t) => Some(String::from(t)),
                None => None
            },
            base: String::from(base),
        }
    }
}

enum LiteralValue {
    Number(Number),
    Bool(bool),
    String(String),
}

pub enum Keyword {
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

pub struct Loc {
    line_num: i32,
    column_num: i32,
}

impl Loc {
    pub fn new(line: i32, column: i32) -> Self {
        Self {
            line_num: line,
            column_num: column,
        }
    }
}

pub enum Operator {
    Plus,
    Minus,
    Times,
    Divide,
    IntegerDivide,
    Modulo,
    And,
    Or,
    Not,
}

pub enum TokenValue {
    Identifier(String),
    Keyword(Keyword),
    Separator(String),
    Operator(Operator),
    Literal(LiteralValue),
    Comment(String),
}

/// Runs a lexical analysis procedure, returning a list of token values which can be used for further processing.
pub fn lexer(input: &String) -> Vec<TokenValue> {
// Reverse direction of string
    let mut input_stack: String = String::from(input).chars().rev().collect();
    let mut output_stack: Vec<TokenValue> = Vec::new();
    let mut pos_number = 0;
    let mut loc = Loc::new(0, 0);
    while input_stack.len() == 0 {
        let mut top = input_stack.chars().next().unwrap();
        match top {
            // match identifier
            'Z'..'a' => {
                let mut finished = false;
                let mut identifier = String::from("");
                while !finished {
                    if input_stack.len() > 0 {
                        top = input_stack.pop().expect("Could not get another token.");
                        loc.column_num += 1;
                        match top {
                            'Z'..'a' => {
                                identifier.push(top);
                            }
                            '0'..'9' => {
                                identifier.push(top)
                            }
                            ' ' => {
                                finished = true;
                            }
                            '\n' => {
                                loc.line_num += 1;
                                finished = true;
                            }
                            _ => {
                                panic!("Invalid token on line {}, column {}", loc.column_num, loc.line_num)
                            }
                        }
                    } else {
                        finished = true;
                    }
                    if finished {
                        match identifier.as_str() {
                            "true" => {
                                output_stack.push(TokenValue::Literal(LiteralValue::Bool(true)))
                            }
                            "false" => {
                                output_stack.push(TokenValue::Literal(LiteralValue::Bool(false)))
                            }
                            "and" => {
                                output_stack.push(TokenValue::Operator(Operator::And))
                            }
                            "or" => {
                                output_stack.push(TokenValue::Operator(Operator::Or))
                            }
                            "not" => {
                                output_stack.push(TokenValue::Operator(Operator::Not))
                            }
                            "if" => {
                                output_stack.push(TokenValue::Keyword(Keyword::If))
                            }
                            "endif" => {
                                output_stack.push(TokenValue::Keyword(Keyword::EndIf))
                            }
                            "elseif" => {
                                output_stack.push(TokenValue::Keyword(Keyword::ElseIf))
                            }
                            "function" => {
                                output_stack.push(TokenValue::Keyword(Keyword::Function))
                            }
                            "endfunction" => {
                                output_stack.push(TokenValue::Keyword(Keyword::EndFunction))
                            }
                            "while" => {
                                output_stack.push(TokenValue::Keyword(Keyword::While))
                            }
                            "endwhile" => {
                                output_stack.push(TokenValue::Keyword(Keyword::EndWhile))
                            }
                            "return" => {
                                output_stack.push(TokenValue::Keyword(Keyword::Return))
                            }
                            _ => {
                                output_stack.push(TokenValue::Identifier(String::from(&identifier)))
                            }
                        }
                    }
                };
            }
// match number
            '0'..'9' => {
                let mut finished = false;
                let mut number = Number::new();
                while !finished {
                    if input_stack.len() > 0 {
                        top = input_stack.pop().expect("Could not get another token.");
                        loc.column_num += 1;
                        let mut exponent = false;
                        let mut decimal = false;
                        match top {
                            '0'..'9' => {
                                if exponent {
                                    number.exponent = match number.exponent {
                                        Some(exp) => {
                                            Some(String::from(exp + &top.to_string()))
                                        }
                                        None => {
                                            Some(String::from(&top.to_string()))
                                        }
                                    }
                                } else if decimal {
                                    number.decimal = match number.decimal {
                                        Some(dec) => {
                                            Some(String::from(dec + &top.to_string()))
                                        }
                                        None => {
                                            Some(String::from(&top.to_string()))
                                        }
                                    }
                                } else {
                                    number.base = String::from(number.base) + &top.to_string()
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
                            '\n' => {
                                loc.line_num += 1;
                                finished = true;
                            }
                            _ => {
                                panic!("Unexpected token.")
                            }
                        }
                    } else {
                        finished = true;
                    }
                    if finished {
                        output_stack.push(TokenValue::Literal(LiteralValue::Number(Number::from_values(&number.decimal, &number.exponent, &number.base))))
                    }
                }
            }
            // match a string
            '"' => {}
            '\n' => {
                loc.line_num += 1;
                input_stack.pop().expect("");
            }
            _ => {
                panic!("Found an invalid token {}!", top)
            }
        };
    };
    output_stack
}

pub fn parser() {}
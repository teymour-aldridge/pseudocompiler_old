#[derive(Debug)]
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

#[derive(Debug)]
enum LiteralValue {
    Number(Number),
    Bool(bool),
    String(String),
}

#[derive(Debug)]
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

#[derive(Debug, Copy, Clone)]
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

#[derive(Debug)]
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

#[derive(Debug)]
pub enum TokenValue {
    Identifier(String, Loc),
    Keyword(Keyword, Loc),
    Separator(String),
    Operator(Operator, Loc),
    Literal(LiteralValue, Loc),
    Comment(String, Loc),
    OpenBracket(Loc),
    CloseBracket(Loc),
}

/// Runs a lexical analysis procedure, returning a list of token values which can be used for further processing.
pub fn lexer(input: &String) -> Vec<TokenValue> {
// Reverse direction of string
    let mut input_stack: String = String::from(input).chars().rev().collect();
    let mut output_stack: Vec<TokenValue> = Vec::new();
    let mut pos_number = 0;
    let mut loc = Loc::new(1, 1);
    while input_stack.len() > 0 {
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
                                output_stack.push(TokenValue::Literal(LiteralValue::Bool(true), loc))
                            }
                            "false" => {
                                output_stack.push(TokenValue::Literal(LiteralValue::Bool(false), loc))
                            }
                            "and" => {
                                output_stack.push(TokenValue::Operator(Operator::And, loc))
                            }
                            "or" => {
                                output_stack.push(TokenValue::Operator(Operator::Or, loc))
                            }
                            "not" => {
                                output_stack.push(TokenValue::Operator(Operator::Not, loc))
                            }
                            "if" => {
                                output_stack.push(TokenValue::Keyword(Keyword::If, loc))
                            }
                            "endif" => {
                                output_stack.push(TokenValue::Keyword(Keyword::EndIf, loc))
                            }
                            "elseif" => {
                                output_stack.push(TokenValue::Keyword(Keyword::ElseIf, loc))
                            }
                            "function" => {
                                output_stack.push(TokenValue::Keyword(Keyword::Function, loc))
                            }
                            "endfunction" => {
                                output_stack.push(TokenValue::Keyword(Keyword::EndFunction, loc))
                            }
                            "while" => {
                                output_stack.push(TokenValue::Keyword(Keyword::While, loc))
                            }
                            "endwhile" => {
                                output_stack.push(TokenValue::Keyword(Keyword::EndWhile, loc))
                            }
                            "return" => {
                                output_stack.push(TokenValue::Keyword(Keyword::Return, loc))
                            }
                            _ => {
                                output_stack.push(TokenValue::Identifier(String::from(&identifier), loc))
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
                        output_stack.push(TokenValue::Literal(LiteralValue::Number(Number::from_values(&number.decimal, &number.exponent, &number.base)), loc))
                    }
                }
            }
            // match a string
            '"' => {}
            '\n' => {
                loc.line_num += 1;
                input_stack.pop().expect("");
            }
            '/' => {
                let mut stream = input_stack.chars();
                let mut next = stream.next().unwrap();
                match next {
                    '/' => {
                        input_stack.pop().unwrap();
                        input_stack.pop().unwrap();
                        loc.column_num += 2;
                        output_stack.push(TokenValue::Operator(Operator::IntegerDivide, loc))
                    }
                    ' ' => {
                        input_stack.pop().unwrap();
                        loc.column_num += 1;
                        output_stack.push(TokenValue::Operator(Operator::Divide, loc))
                    }
                    '0'..'9' => {
                        input_stack.pop().unwrap();
                        loc.column_num += 1;
                        output_stack.push(TokenValue::Operator(Operator::Divide, loc))
                    }
                    _ => {
                        panic!("Invalid token following a '/' at line {}, column {}", loc.line_num, loc.column_num)
                    }
                }
            }
            '+' => {
                input_stack.pop().unwrap();
                loc.column_num += 1;
                output_stack.push(TokenValue::Operator(Operator::Plus, loc))
            }
            '-' => {
                input_stack.pop().unwrap();
                loc.column_num += 1;
                output_stack.push(TokenValue::Operator(Operator::Minus, loc))
            }
            '*' => {
                input_stack.pop().unwrap();
                loc.column_num += 1;
                output_stack.push(TokenValue::Operator(Operator::Times, loc))
            }
            '(' => {
                input_stack.pop().unwrap();
                loc.column_num += 1;
                output_stack.push(TokenValue::OpenBracket(loc))
            }
            ')' => {
                input_stack.pop().unwrap();
                loc.column_num += 1;
                output_stack.push(TokenValue::CloseBracket(loc))
            }
            _ => {
                panic!("Found an invalid token {} at line {}, column {}.", top, loc.line_num, loc.column_num)
            }
        };
    };
    output_stack
}

pub fn parser() {}
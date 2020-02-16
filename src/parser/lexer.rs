use crate::parser::helpers::NumberState;

use super::helpers;
use crate::parser::lexer::Token::Literal;

#[derive(Debug, PartialEq, Clone)]
pub struct Number {
    pub exponent: Option<String>,
    pub decimal: Option<String>,
    pub base: String,
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
                None => None,
            },
            decimal: match decimal {
                Some(t) => Some(String::from(t)),
                None => None,
            },
            base: String::from(base),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum LiteralValue {
    Number(Number),
    Bool(bool),
    String(String),
}

#[derive(Debug, PartialEq, Clone)]
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
    Do,
    To,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Loc {
    pub line_num: i32,
    pub column_num: i32,
}

impl Loc {
    pub fn new(line: i32, column: i32) -> Self {
        Self {
            line_num: line,
            column_num: column,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
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
    Equals,
    In,
    NotEquals,
    // Not used as part of the lexer (only in the parser)
    Empty,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Identifier(String),
    Keyword(Keyword),
    Separator(String),
    Operator(Operator),
    FunctionCall(String),
    Literal(LiteralValue),
    Comment(String),
    OpenBracket,
    CloseBracket,
    OpenSquareBracket,
    CloseSquareBracket,
    Comma,
    Dot,
    EndOfSequence,
    NewLine,
}

#[derive(Debug, PartialEq, Clone)]
pub struct TokenValue {
    pub token: Token,
    pub loc: Loc,
}

impl TokenValue {
    pub fn new(token: Token, line: i32, col: i32) -> Self {
        Self {
            token,
            loc: Loc::new(line, col),
        }
    }
}

pub fn get_next(input: &mut String) -> char {
    let first = input.chars().next().unwrap();
    input.remove(0);
    first
}

/// Runs a lexical analysis procedure, returning a list of token values which can be used for further processing.
pub fn lexer(input: &String) -> Vec<TokenValue> {
    // Reverse direction of string
    let mut input_stack: String = String::from(input);
    let mut output_stack: Vec<TokenValue> = Vec::new();
    let mut pos_number = 0;
    let mut loc = Loc::new(1, 0);
    while input_stack.len() > 0 {
        let mut top = input_stack.chars().next().unwrap();
        match top {
            // match identifier
            'a'..='z' | 'A'..='Z' => {
                let mut finished = false;
                let mut identifier = String::from("");
                while !finished {
                    if input_stack.len() > 0 {
                        top = get_next(&mut input_stack);
                        loc.column_num += 1;
                        match top {
                            'a'..='z' | 'A'..='Z' => {
                                identifier.push(top);
                            }
                            '0'..'9' => identifier.push(top),
                            ' ' => {
                                finished = true;
                            }
                            '/' | '+' | '-' | '*' | '.' | '=' => {
                                input_stack = top.to_string() + &input_stack;
                                finished = true;
                            }
                            '\n' => {
                                loc.line_num += 1;
                                finished = true;
                            }
                            _ => panic!(
                                "Unexpected token {} on line {}, column {}.",
                                top, loc.column_num, loc.line_num
                            ),
                        }
                    } else {
                        finished = true;
                    }
                    if finished {
                        match identifier.as_str() {
                            "true" => output_stack.push(TokenValue::new(
                                Token::Literal(LiteralValue::Bool(true)),
                                loc.line_num,
                                loc.column_num,
                            )),
                            "false" => output_stack.push(TokenValue::new(
                                Token::Literal(LiteralValue::Bool(false)),
                                loc.line_num,
                                loc.column_num,
                            )),
                            "and" => output_stack.push(TokenValue::new(
                                Token::Operator(Operator::And),
                                loc.line_num,
                                loc.column_num,
                            )),
                            "or" => output_stack.push(TokenValue::new(
                                Token::Operator(Operator::Or),
                                loc.line_num,
                                loc.column_num,
                            )),
                            "not" => output_stack.push(TokenValue::new(
                                Token::Operator(Operator::Not),
                                loc.line_num,
                                loc.column_num,
                            )),
                            "if" => output_stack.push(TokenValue::new(
                                Token::Keyword(Keyword::If),
                                loc.line_num,
                                loc.column_num,
                            )),
                            "endif" => output_stack.push(TokenValue::new(
                                Token::Keyword(Keyword::EndIf),
                                loc.line_num,
                                loc.column_num,
                            )),
                            "elseif" => output_stack.push(TokenValue::new(
                                Token::Keyword(Keyword::ElseIf),
                                loc.line_num,
                                loc.column_num,
                            )),
                            "function" => output_stack.push(TokenValue::new(
                                Token::Keyword(Keyword::Function),
                                loc.line_num,
                                loc.column_num,
                            )),
                            "endfunction" => output_stack.push(TokenValue::new(
                                Token::Keyword(Keyword::EndFunction),
                                loc.line_num,
                                loc.column_num,
                            )),
                            "return" => output_stack.push(TokenValue::new(
                                Token::Keyword(Keyword::EndIf),
                                loc.line_num,
                                loc.column_num,
                            )),
                            "while" => output_stack.push(TokenValue::new(
                                Token::Keyword(Keyword::While),
                                loc.line_num,
                                loc.column_num,
                            )),
                            "endwhile" => output_stack.push(TokenValue::new(
                                Token::Keyword(Keyword::EndWhile),
                                loc.line_num,
                                loc.column_num,
                            )),
                            "for" => output_stack.push(TokenValue::new(
                                Token::Keyword(Keyword::For),
                                loc.line_num,
                                loc.column_num,
                            )),
                            "endfor" => output_stack.push(TokenValue::new(
                                Token::Keyword(Keyword::EndFor),
                                loc.line_num,
                                loc.column_num,
                            )),
                            "to" => output_stack.push(TokenValue::new(
                                Token::Keyword(Keyword::Do),
                                loc.line_num,
                                loc.column_num,
                            )),
                            "do" => output_stack.push(TokenValue::new(
                                Token::Keyword(Keyword::Do),
                                loc.line_num,
                                loc.column_num,
                            )),
                            _ => output_stack.push(TokenValue::new(
                                Token::Identifier(String::from(&identifier)),
                                loc.line_num,
                                loc.column_num,
                            )),
                        }
                    }
                }
            }
            // match number
            '0'..'9' => {
                let mut finished = false;
                let mut number = Number::new();
                let mut state = NumberState::new();
                while !finished {
                    if input_stack.len() > 0 {
                        top = get_next(&mut input_stack);
                        loc.column_num += 1;
                        match top {
                            '0'..'9' => {
                                if state.decimal {
                                    number.decimal = match number.decimal {
                                        Some(dec) => Some(String::from(dec + &top.to_string())),
                                        None => Some(String::from(&top.to_string())),
                                    };
                                } else if state.exponent {
                                    number.exponent = match number.exponent {
                                        Some(exp) => Some(String::from(exp + &top.to_string())),
                                        None => Some(String::from(&top.to_string())),
                                    }
                                } else {
                                    number.base = String::from(number.base) + &top.to_string()
                                }
                            }
                            '.' => {
                                match input_stack.chars().next().unwrap() {
                                    '0'..='9' => {}
                                    _ => panic!(
                                        "No digit after the '.' on line {}, column {}.",
                                        loc.line_num, loc.column_num
                                    ),
                                };
                                state.set_dec(true);
                            }
                            'e' => {
                                match input_stack.chars().next().unwrap() {
                                    '0'..='9' => {}
                                    _ => panic!(
                                        "No digit after the 'e' on line {}, column {}.",
                                        loc.line_num, loc.column_num
                                    ),
                                };
                                state.set_exp(true);
                            }
                            ' ' => {
                                break;
                            }
                            '\n' => {
                                loc.line_num += 1;
                                finished = true;
                            }
                            _ => panic!(
                                "Unexpected token {} on line {}, column {}.",
                                top, loc.column_num, loc.line_num
                            ),
                        }
                    } else {
                        finished = true;
                    }
                    if finished {
                        output_stack.push(TokenValue::new(
                            Token::Literal(LiteralValue::Number(Number::from_values(
                                &number.exponent,
                                &number.decimal,
                                &number.base,
                            ))),
                            loc.line_num,
                            loc.column_num,
                        ));
                    }
                }
            }
            // match a string
            '"' => {}
            '\n' => {
                output_stack.push(TokenValue::new(
                    Token::NewLine,
                    loc.line_num,
                    loc.column_num,
                ));
                loc.line_num += 1;
                get_next(&mut input_stack);
            }
            '/' => {
                get_next(&mut input_stack);
                let next = input_stack.chars().next().unwrap();
                match next {
                    '/' => output_stack.push(TokenValue::new(
                        Token::Operator(Operator::IntegerDivide),
                        loc.line_num,
                        loc.column_num,
                    )),
                    _ => output_stack.push(TokenValue::new(
                        Token::Operator(Operator::Divide),
                        loc.line_num,
                        loc.column_num,
                    )),
                }
            }
            '+' => {
                get_next(&mut input_stack);
                loc.column_num += 1;
                output_stack.push(TokenValue::new(
                    Token::Operator(Operator::Plus),
                    loc.line_num,
                    loc.column_num,
                ))
            }
            '%' => {
                get_next(&mut input_stack);
                loc.column_num += 1;
                output_stack.push(TokenValue::new(
                    Token::Operator(Operator::Modulo),
                    loc.line_num,
                    loc.column_num,
                ))
            }
            '-' => {
                get_next(&mut input_stack);
                loc.column_num += 1;
                output_stack.push(TokenValue::new(
                    Token::Operator(Operator::Minus),
                    loc.line_num,
                    loc.column_num,
                ))
            }
            '*' => {
                get_next(&mut input_stack);
                loc.column_num += 1;
                output_stack.push(TokenValue::new(
                    Token::Operator(Operator::Times),
                    loc.line_num,
                    loc.column_num,
                ))
            }
            '(' => {
                get_next(&mut input_stack);
                loc.column_num += 1;
                output_stack.push(TokenValue::new(
                    Token::OpenBracket,
                    loc.line_num,
                    loc.column_num,
                ))
            }
            ')' => {
                get_next(&mut input_stack);
                loc.column_num += 1;
                output_stack.push(TokenValue::new(
                    Token::CloseBracket,
                    loc.line_num,
                    loc.column_num,
                ))
            }
            ' ' => {
                get_next(&mut input_stack);
            }
            ',' => {
                get_next(&mut input_stack);
                output_stack.push(TokenValue::new(Token::Comma, loc.line_num, loc.column_num))
            }
            '.' => {
                get_next(&mut input_stack);
                output_stack.push(TokenValue::new(Token::Dot, loc.line_num, loc.column_num))
            }
            '=' => {
                get_next(&mut input_stack);
                loc.column_num += 1;
                output_stack.push(TokenValue::new(
                    Token::Operator(Operator::Equals),
                    loc.line_num,
                    loc.column_num,
                ))
            }
            '!' => {
                get_next(&mut input_stack);
                let next = get_next(&mut input_stack);
                match next {
                    '=' => {
                        loc.column_num += 1;
                        output_stack.push(TokenValue::new(
                            Token::Operator(Operator::NotEquals),
                            loc.line_num,
                            loc.column_num,
                        ))
                    }
                    _ => {
                        panic!(
                            "Expected an equals following a '!' on line {}, column {}.",
                            loc.line_num, loc.column_num
                        );
                    }
                }
                loc.column_num += 1;
                output_stack.push(TokenValue::new(
                    Token::Operator(Operator::Equals),
                    loc.line_num,
                    loc.column_num,
                ))
            }
            _ => panic!(
                "Found an invalid token {} at line {}, column {}.",
                top, loc.line_num, loc.column_num
            ),
        };
    }
    output_stack.push(TokenValue::new(
        Token::EndOfSequence,
        loc.line_num,
        loc.column_num,
    ));
    output_stack
}

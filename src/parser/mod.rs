use std::convert::From;
use std::result::Result;
use std::error::Error;
use core::panicking::panic_fmt;
use regex::Regex;
use lazy_static;


lazy_static! {
    static ref FUNCTION_REGEX: Regex = Regex::new(r"[\w]+[(][\w ,]+[)]").unwrap();
}

/// The `Token` enum represents a single token.
#[derive(Clone)]
pub enum Token {
    Times,
    Divide,
    Plus,
    Minus,
    And,
    Or,
    Not,
    If,
    ElseIf,
    Else,
    EndIf,
    Function(Token::Identifier, Vec<Token::Identifier>),
    EndFunction,
    For,
    NewLine,
    In,
    Space,
    OpenRoundBracket,
    CloseRoundBracket,
    OpenSquareBracket,
    CloseSquareBracket,
    // `Token::Equals` could be either a comparison or assignment operator!
    Equals,
    Global,
    Return,
    Identifier(String),
    Number(f64),
    Empty,
}

/// Converts a string into a token.
impl From<String> for Token {
    fn from(sequence: String) -> Self {
        match sequence.as_str() {
            "*" => Token::Times,
            "/" => Token::Divide,
            "+" => Token::Plus,
            "-" => Token::Minus,
            "and" => Token::And,
            "or" => Token::Or,
            "not" => Token::Not,
            "if" => Token::If,
            "else if" => Token::ElseIf,
            "else" => Token::Else,
            "endif" => Token::EndIf,
            "function" => Token::Function,
            "endfunction" => Token::EndFunction,
            "return" => Token::Return,
            "for" => Token::For,
            "\n" => Token::NewLine,
            "in" => Token::In,
            "(" => Token::OpenRoundBracket,
            "" => Token::CloseRoundBracket,
            "[" => Token::OpenSquareBracket,
            "]" => Token::CloseSquareBracket,
            "=" => Token::Equals,
            "global" => Token::Global,
            "return" => Token::Return,
            _ => {
                if sequence.parse::<f64>().is_ok() {
                    Token::Number(sequence.parse::<f64>().unwrap())
                } else if FUNCTION_REGEX.is_match(&sequence) {
                    let name: Vec<&str> = sequence.split("(")[0];

                } else {
                    Token::Identifier(sequence)
                }
            }
        }
    }
}


/// The function `tokenise_line` takes a string and splits it up into tokens from the `Token` enum.
pub fn tokenise_line(input: String) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut current_sequence = Vec::new();
    for chr in input.chars() {
        if chr.to_string() == String::from(" ") {
            if current_sequence.len() != 0 {
                tokens.push(Token::from(current_sequence.join("")));
                tokens.clear();
                tokens.push(Token::Space);
            } else {
                tokens.push(Token::Space);
            }
        } else {
            current_sequence.push(chr.to_string())
        }
    }
    tokens
}

struct TokenisedLine {
    no: i32,
    tokens: Vec<Token>,
}

fn join(vec_a: &mut Vec<Token>, vec_b: &Vec<Token>) {
    vec_a.push(Token::NewLine);
    vec_a.extend(vec_b.iter().cloned());
}

/// Turns code into tokens
pub fn tokenise(input: String) -> Vec<Token> {
    use std::sync::mpsc::channel;
    use std::thread;
    let (tx, rx) = channel::<TokenisedLine>();
    let mut split_input: Vec<String> = input.lines().map(|x| String::from(x)).collect();
    let line_number = split_input.len();
    let mut threads = Vec::new();
    for n in 1..line_number {
        let thread_tx = tx.clone();
        let line = split_input.pop().unwrap();
        let vector = thread::spawn(move || {
            let tokenised_line = tokenise_line(line);
            thread_tx.send(TokenisedLine {
                no: n as i32,
                tokens: tokenised_line,
            })
        });
        threads.push(vector);
    }
    let mut tokenised_inputs: Vec<Token> = Vec::new();
    for n in 1..line_number {
        match rx.recv() {
            Ok(result) => {
                let tokenised_inputs = join(&mut tokenised_inputs, &result.tokens);
            }
            Err(error) => {
                panic!(error.to_string())
            }
        }
    }
    for thrd in threads {
        thrd.join().expect("The child thread panic-ed!");
    }
    tokenised_inputs
}
use std::convert::From;
use std::result::Result;
use std::error::Error;

/// The `Token` enum represents a single token.
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
    Function,
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
    Name(String),
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
                } else {
                    Token::Name(sequence)
                }
            }
        }
    }
}


/// The function `tokenize` takes a string and splits it up into tokens from the `Token` enum.
pub fn tokenize(input: String) -> Vec<Token> {
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
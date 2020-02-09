use crate::parser::lexer::{Keyword, Token, TokenValue};
use std::process::Output;

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref FUNCTION: Regex = Regex::new(r"function [\w\W]+\([\w ,]+\)").unwrap();
    static ref WHILE: Regex = Regex::new(r"while [\w\W]+");
    static ref ENDBLOCK: Regex = Regex::new(r"end[\w+]+").unwrap();
}

pub fn to_js(str: String) -> String {
    let mut string: String = String::from(str);
    FUNCTION.replace(&mut string, "$& {");
    ENDBLOCK.replace(&mut string, "}");
    string
}

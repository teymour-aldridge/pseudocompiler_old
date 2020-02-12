use crate::parser::lexer::{Keyword, Token, TokenValue};
use std::process::Output;

use lazy_static::lazy_static;
use regex::Regex;
/*
lazy_static! {
    static ref FUNCTION: Regex = Regex::new(r"function [\w]+\([\w ,]+\)").unwrap();
    static ref WHILE: Regex = Regex::new(r"while [\w]+");
    static ref FOR: Regex = Regex::new(r"for ([\w]+)\=([\w]+) to ([\w]+)");
    static ref ENDBLOCK: Regex = Regex::new(r"end[\w+]+").unwrap();
}

pub fn to_js(str: String) -> String {
    let mut string: String = String::from(str);
    FUNCTION.replace(&mut string, "$& {");
    WHILE.replace(&mut string, "$& {");
    FOR.replace(&mut string, "for (\1=\2; \1<\3; \1++) {");
    ENDBLOCK.replace(&mut string, "}");
    string
}
*/

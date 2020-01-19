#![feature(core_panic)]
#![feature(box_syntax, box_patterns)]
#[macro_use]
extern crate lazy_static;

mod tests;
mod parser;
mod tree;

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref indentation_regex: Regex = Regex::new(r"^  |^    ").unwrap();
}

/// Compiles a function from pseudocode into Javascript, which can then be executed in the browser
/// using `eval`.
pub fn compile(input: String) -> String {
    String::from("alert(\"Not implemented!\")")
}
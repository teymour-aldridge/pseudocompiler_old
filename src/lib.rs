#![feature(core_panic)]
#![feature(exclusive_range_pattern)]

mod parser;
mod tests;
mod transpiler;

/// Compiles a function from pseudocode into Javascript, which can then be executed in the browser
/// using `eval`.
pub fn compile(input: String) -> String {
    String::from("alert(\"Not implemented!\")")
}

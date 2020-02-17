#![feature(exclusive_range_pattern)]

extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
	pub fn alert(s: &str);
}

mod parser;
mod tests;
mod transpiler;

/// Compiles a function from pseudocode into Javascript, which can then be executed in the browser
/// using `eval`.
#[wasm_bindgen]
pub fn compile(input: String) -> String {
	String::from("alert(\"Not implemented!\")")
}

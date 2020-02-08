use indextree::Arena;
use crate::parser::lexer::{Loc, Operator};

pub enum Condition {
    Compare(Operator)
}

pub enum Item {
    Function(Loc),
    // The `String` is the name of the function being called.
    Call(String, Loc),
    Assign(Loc),
    Operator(Operator, Loc),
    If(Loc),
    ElseIf(Loc),
    Condition(Condition, Loc),
    // The `String` is the variable name
    Variable(String, Loc),
}

fn parse_expression() {}

fn parse_if() {}

fn parse_function() {}

fn parse_while() {}

fn parse_for() {}

pub fn lexer() {
    let arena = &mut Arena::new();
}
use crate::parser::lexer::{Keyword, Loc, Operator, Token, TokenValue};
use indextree::Arena;
use std::any::Any;

pub enum Condition {
    Compare(Operator),
}

pub enum Item {
    Function,
    // The `String` is the name of the function being called.
    Call(String),
    Assign,
    Operator(Operator),
    If,
    ElseIf,
    Condition(Condition),
    // The `String` is the variable name
    Variable(String),
}

pub struct Node {
    loc: Loc,
    item: Item,
}

fn parse_boolean() {}

fn parse_arithmetic() {}

fn parse_expression() {}

fn parse_if(parent: &mut Arena<Node>, tokens: &mut Vec<TokenValue>) {}

fn parse_function() {}

fn parse_while() {}

fn parse_for() {}

fn parse_assignment() {}

fn parse_statement(parent: &mut Arena<Node>, tokens: &mut Vec<TokenValue>) {
    let mut lexitem = tokens.iter().next().unwrap();
    match lexitem.token {
        Token::Keyword(Keyword::If) => {
            parse_if(parent, tokens);
        }
        _ => panic!(
            "Unexpected term on line {}, column {}.",
            lexitem.loc.line_num, lexitem.loc.column_num
        ),
    }
}

pub fn lexer() {
    let arena: &Arena<Node> = &mut Arena::new();
}

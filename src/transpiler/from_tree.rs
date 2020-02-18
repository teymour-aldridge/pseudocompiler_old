use crate::parser::{parser, parser::Node};
use indextree::{Arena, NodeId};

fn transpile_block() {}

fn transpile_if() {}

fn transpile_expression() {}

fn transpile_function() {}

fn transpile_while() {}

fn transpile_for() {}

pub fn from_tree(tree: Arena<Node>, start_node: NodeId) -> String {
    let mut output = String::new();
    output
}

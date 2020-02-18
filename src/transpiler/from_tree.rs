use crate::parser::{parser, parser::Node};
use indextree::{Arena, NodeId};

fn transpile_block(block_node: &NodeId, arena: &mut Arena<Node>, output: &mut String) {}

fn transpile_if(if_node: &NodeId, arena: &mut Arena<Node>, output: &mut String) {}

fn transpile_expression(expression_node: &NodeId, arena: &mut Arena<Node>, output: &mut String) {}

fn transpile_function(function_node: &NodeId, arena: &mut Arena<Node>, output: &mut String) {}

fn transpile_while(while_node: &NodeId, arena: &mut Arena<Node>, output: &mut String) {}

fn transpile_for(for_node: &NodeId, arena: &mut Arena<Node>, output: &mut String) {}

pub fn from_tree(tree: Arena<Node>, start_node: NodeId) -> String {
	let mut output = String::new();
	output
}

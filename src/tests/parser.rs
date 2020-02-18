use crate::parser::*;
use crate::parser::lexer::Number;

#[test]
fn test_parser() {
    let (tree, initial_node) = parser::parser(&mut lexer::lexer(&String::from("x=1")));
    let x = tree.get(initial_node).unwrap();
    let assign_operator_node = tree.get(x.first_child().unwrap()).unwrap().get();
    assert!(assign_operator_node.item == parser::Item::Assign);
    let assign_operator = x.first_child().unwrap();
    let identifier_node = tree.get(assign_operator).unwrap().first_child().unwrap();
    let identifier = tree.get(identifier_node).unwrap().get();
    assert!(identifier.item == parser::Item::Identifier(String::from("x")));
    let operand_node = tree.get(assign_operator).unwrap().last_child().unwrap();
    let operand = tree.get(operand_node).unwrap().get();
    assert!(operand.item == parser::Item::Expression);
}
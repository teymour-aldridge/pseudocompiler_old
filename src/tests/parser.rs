use crate::parser::{lexer::Number, *};

#[test]
fn test_assignment() {
	let (arena, initial_node) = parser::parser(&mut lexer::lexer(&String::from("x=1")));
	let x = arena.get(initial_node).unwrap();
	let assign_operator_node = arena.get(x.first_child().unwrap()).unwrap().get();
	assert!(assign_operator_node.item == parser::Item::Assign);
	let assign_operator = x.first_child().unwrap();
	let identifier_node = arena.get(assign_operator).unwrap().first_child().unwrap();
	let identifier = arena.get(identifier_node).unwrap().get();
	assert!(identifier.item == parser::Item::Identifier(String::from("x")));
	let expression_node = arena.get(assign_operator).unwrap().last_child().unwrap();
	let expression = arena.get(expression_node).unwrap().get();
	assert!(expression.item == parser::Item::Expression);
	let number_node = arena.get(expression_node).unwrap().last_child().unwrap();
	let number = arena.get(number_node).unwrap().get();
	assert!(
		number.item
			== parser::Item::Number(Number {
				base: String::from("1"),
				exponent: None,
				decimal: None,
			})
	);
}

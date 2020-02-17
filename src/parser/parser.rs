use crate::parser::lexer::{Keyword, LiteralValue, Loc, Number, Operator, Token, TokenValue};
use indextree::{Arena, NodeId};

pub enum Item {
	Function,
	ParameterList(Vec<Item>),
	// The `String` is the name of the function being called.
	Call(String),
	Assign,
	Operator(Operator),
	If,
	ElseIf,
	// The `String` is the identifier name
	Identifier(String),
	Expression,
	Number(Number),
	While,
	For,
	Body,
	Program,
	Block,
}

pub fn priority(o: &TokenValue) -> u32 {
	match &o.token {
		Token::Operator(o) => {
			match o {
				Operator::Times | Operator::Divide | Operator::IntegerDivide | Operator::Modulo => {
					5
				}
				Operator::Plus | Operator::Minus => 4,
				Operator::Equals | Operator::NotEquals => 3,
				Operator::And => 2,
				Operator::Or => 1,
				Operator::Not => 5,
				// This is kept here in case more operators are to be added
				_ => 0,
			}
		}
		_ => panic!("Not an operator."),
	}
}

pub fn left_associative(o: &TokenValue) -> bool {
	match &o.token {
		Token::Operator(o) => {
			match o {
				Operator::Times => true,
				Operator::Divide => true,
				Operator::IntegerDivide => true,
				Operator::Plus => true,
				Operator::Minus => true,
				Operator::Equals => true,
				Operator::NotEquals => true,
				Operator::Modulo => true,
				Operator::And => true,
				Operator::Or => true,
				Operator::Not => false,
				// This is kept here in case more operators are to be added
				_ => true,
			}
		}
		_ => panic!("Not an operator."),
	}
}

pub fn is_operator(o: &TokenValue) -> bool {
	match o.token {
		Token::FunctionCall(_) | Token::Operator(_) => true,
		_ => false,
	}
}

pub struct Node {
	loc: Loc,
	item: Item,
}

impl Node {
	pub fn new(item: Item, loc: Loc) -> Self {
		Self { item, loc }
	}
}

fn parse_expression(parent: &NodeId, arena: &mut Arena<Node>, tokens: &mut Vec<TokenValue>) {
	let mut operator_stack: Vec<TokenValue> =
		vec![TokenValue::new(Token::Operator(Operator::Empty), 0, 0)];
	let mut output_stack: Vec<TokenValue> = Vec::new();
	let mut finished = false;
	while !finished {
		let next: TokenValue = tokens.pop().unwrap();
		match &next.token {
			Token::Operator(o) => {
				if priority(&next) > priority(operator_stack.get(0).unwrap()) {
					operator_stack.push(next)
				} else {
					while priority(&next) <= priority(operator_stack.get(0).unwrap()) {
						output_stack.push(operator_stack.pop().unwrap());
					}
					operator_stack.push(next);
				}
			}
			Token::Literal(LiteralValue::Number(n)) => {}
			Token::Keyword(Keyword::Function) => {
				let identifier = tokens.pop().unwrap();
				match identifier.token {
					Token::Identifier(name) => {
						operator_stack.push(TokenValue::new(
							Token::FunctionCall(name),
							identifier.loc.line_num,
							identifier.loc.column_num,
						));
					}
					_ => panic!(
						"Expected an identifier after the 'function' keyword on line {}, \
						 column {}.",
						identifier.loc.line_num, identifier.loc.column_num
					),
				}
				let open_bracket = tokens.pop().unwrap();
				match open_bracket.token {
					Token::OpenBracket => {
						operator_stack.push(open_bracket);
					}
					_ => panic!(
						"Expected an opening bracket after the 'function' keyword on \
						 line {}, column {}.",
						open_bracket.loc.line_num, open_bracket.loc.column_num
					),
				}
				let mut all_arguments = false;
				while !all_arguments {
					let argument = tokens.pop().unwrap();
					match &argument.token {
						Token::Identifier(_) | Token::Literal(LiteralValue::Number(_)) => {
							output_stack.push(argument);
						}
						Token::Comma => {}
						Token::EndOfSequence => panic!(
							"Expected a closing bracket after the function call on line {}, \
							 column {}.",
							next.loc.line_num, next.loc.column_num
						),
						Token::CloseBracket => {
							output_stack.push(argument);
							all_arguments = true;
						}
						_ => panic!(
							"Unexpected token after the 'function' keyword on line {}, \
							 column {}.",
							argument.loc.line_num, argument.loc.column_num
						),
					}
				}
			}
			Token::OpenBracket => {
				operator_stack.push(next);
			}
			Token::CloseBracket => {
				while operator_stack.get(0).unwrap().token != Token::OpenBracket {
					output_stack.push(operator_stack.pop().unwrap());
				}
				operator_stack.pop().unwrap();
			}
			Token::EndOfSequence => {
				finished = true;
			}
			_ => panic!("Invalid token found in an expression on line {}, column {}"),
		}
	}
	let mut stack: Vec<NodeId> = Vec::new();
	let mut removed_items = 0;
	for (i, item) in output_stack.clone().iter().enumerate() {
		if !(is_operator(&item)) {
			stack.push(arena.new_node(Node::new(
				match &item.token {
					Token::Identifier(s) => Item::Identifier(s.to_string()),
					Token::Literal(LiteralValue::Number(n)) => Item::Number(n.clone()),
					_ => panic!(
						"Invalid token on line {}, column {}",
						item.loc.line_num, item.loc.column_num
					),
				},
				item.loc,
			)))
		} else {
			match &item.token {
				Token::FunctionCall(s) => {
					let to_pop = i - removed_items;
					let mut nodes: Vec<NodeId> = Vec::new();
					for i in 1..to_pop {
						nodes.push(stack.pop().unwrap())
					}
					let function_call =
						arena.new_node(Node::new(Item::Call(String::from(s)), item.loc));
					for item in nodes {
						function_call.append(item, arena)
					}
				}
				Token::Operator(o) => {
					let operator_node =
						arena.new_node(Node::new(Item::Operator(o.clone()), item.loc));
					operator_node.append(stack.pop().unwrap(), arena);
					operator_node.append(stack.pop().unwrap(), arena);
				}
				_ => {}
			}
		}
	}
	parent.append(stack.pop().unwrap(), arena);
}

fn parse_if(parent: &NodeId, arena: &mut Arena<Node>, tokens: &mut Vec<TokenValue>) {
	let mut then_token = false;
	let mut expression: Vec<TokenValue> = Vec::new();
	let start_loc = tokens.iter().next().unwrap().clone();
	while !then_token {
		let next = tokens.pop().unwrap();
		match next.token {
			Token::Keyword(Keyword::Then) => {
				then_token = true;
				let new_line = tokens.pop().unwrap();
				match new_line.token {
					Token::NewLine => {}
					_ => {
						panic!("Expected a new line following the 'then' keyword on line {}, column {}", new_line.loc.line_num, new_line.loc.column_num);
					}
				}
			}
			_ => {
				expression.push(next);
			}
		}
	}
	let expression_node = arena.new_node(Node::new(Item::Expression, start_loc.loc));
	parent.append(expression_node, arena);
	parse_expression(&expression_node, arena, &mut expression);
}

fn parse_while(parent: &NodeId, arena: &mut Arena<Node>, tokens: &mut Vec<TokenValue>) {
	let n = arena.new_node(Node::new(Item::While, arena.get(*parent).unwrap().get().loc));
	parent.append(n, arena);
	let mut do_token_found = false;
	let mut expression: Vec<TokenValue> = Vec::new();
	while !do_token_found {
		let do_token = tokens.pop().unwrap();
		match do_token.token {
			Token::Keyword(Keyword::Do) => {
				do_token_found = true;
			}
			Token::EndOfSequence => panic!(
				"Expected a 'do' token following the while loop on line {}, column {}.",
				do_token.loc.line_num, do_token.loc.column_num
			),
			Token::NewLine => panic!(
				"Expected a 'do' token following the while loop on line {}, column {}.",
				do_token.loc.line_num, do_token.loc.column_num
			),
			_ => {
				expression.push(do_token);
			}
		}
	}
	parse_expression(&n, arena, &mut expression);
	let end_token = tokens.pop().unwrap();
	if end_token.token != Token::Keyword(Keyword::EndWhile) {
		panic!(
			"Expected the keyword 'endwhile' on line {}, column {}",
			end_token.loc.line_num, end_token.loc.column_num
		)
	}
}

fn parse_for(parent: &NodeId, arena: &mut Arena<Node>, tokens: &mut Vec<TokenValue>) {
	let for_node = arena.new_node(Node::new(Item::For, arena.get(*parent).unwrap().get().loc));
	parent.append(for_node, arena);

	let mut count_variable = String::new();
	let identifier = tokens.pop().unwrap();
	match identifier.token {
		Token::Identifier(s) => {
			count_variable = s;
		}
		_ => panic!(
			"Expected a variable after the keyword 'for' on line {}, column {}",
			identifier.loc.line_num, identifier.loc.column_num
		),
	}

	let equals_sign = tokens.pop().unwrap();
	let mut is_count = false;
	match equals_sign.token {
        Token::Operator(Operator::Equals) => {
            is_count = true;
        }
        Token::Operator(Operator::In) => {
            is_count = false;
        }
        _ => panic!(
            "Expected an equals sign or the 'in' keyword after the variable {} in the for loop on line {}, column {}",
            count_variable, equals_sign.loc.line_num, equals_sign.loc.column_num
        ),
    }
	if is_count {
		let mut expression_block_1: Vec<TokenValue> = Vec::new();
		let mut e_1 = false;
		while !e_1 {
			let next_token = tokens.pop().unwrap();
			match next_token.token {
				Token::Keyword(Keyword::To) => {
					e_1 = true;
				}
				Token::NewLine | Token::EndOfSequence => panic!(
					"Expected the 'to' keyword following 'for <variable>=<expression>' on \
					 line {}, column {}.",
					next_token.loc.line_num, next_token.loc.column_num
				),
				_ => expression_block_1.push(next_token),
			}
		}
		let mut e_2 = false;
		let mut expression_block_2: Vec<TokenValue> = Vec::new();
		while !e_2 {
			let next_token = tokens.pop().unwrap();
			match next_token.token {
				Token::Keyword(Keyword::Do) => {
					e_2 = true;
				}
				Token::NewLine | Token::EndOfSequence => panic!(
					"Expected the 'do' keyword following 'for <variable>=<expression> to \
					 <expression>' on line {}, column {}.",
					next_token.loc.line_num, next_token.loc.column_num
				),
				_ => expression_block_2.push(next_token),
			}
		}
		parse_expression(&for_node, arena, &mut expression_block_1);
		parse_expression(&for_node, arena, &mut expression_block_2);
		parse_block(1, &for_node, arena, tokens)
	} else {
		let mut expression: Vec<TokenValue> = Vec::new();
		let mut found_do = false;
		while !found_do {
			let next_token = tokens.pop().unwrap();
			match next_token.token {
				Token::Keyword(Keyword::Do) => {
					found_do = true;
				}
				Token::EndOfSequence | Token::NewLine => panic!(
					"Expected the 'to' keyword following 'for <variable>=<expression>' on \
					 line {}, column {}.",
					next_token.loc.line_num, next_token.loc.column_num
				),
				_ => {
					expression.push(next_token);
				}
			}
		}
		parse_expression(&for_node, arena, &mut expression);
		parse_block(1, &for_node, arena, tokens)
	}
}

fn parse_function(parent: &NodeId, arena: &mut Arena<Node>, tokens: &mut Vec<TokenValue>) {
	let function_node =
		arena.new_node(Node::new(Item::Function, tokens.iter().next().unwrap().loc));
	parent.append(function_node, arena);
	let identifier_token = tokens.pop().unwrap();
	match identifier_token.token {
		Token::Identifier(s) => {
			let identifier_node =
				arena.new_node(Node::new(Item::Identifier(s), identifier_token.loc));
			function_node.append(identifier_node, arena);
		}
		_ => {
			panic!(
				"Expected an identifier after the 'function' keyword on line {}, column {}.",
				identifier_token.loc.line_num, identifier_token.loc.column_num
			);
		}
	}
	let open_bracket = tokens.pop().unwrap();
	match open_bracket.token {
		Token::OpenBracket => {}
		_ => {
			panic!(
				"Expected an opening bracket after the function declaration on line {}, column {}.",
				open_bracket.loc.line_num, open_bracket.loc.column_num
			);
		}
	}
	let mut parameter_list: Vec<Item> = Vec::new();
	loop {
		let next_token = tokens.pop().unwrap();
		match next_token.token {
			Token::CloseBracket => break,
			Token::Identifier(s) => parameter_list.push(Item::Identifier(s)),
			Token::Comma => {}
			_ => {
				panic!(
					"Unexpected token on line {}, column {}.",
					next_token.loc.line_num, next_token.loc.column_num
				);
			}
		}
	}
	let parameter_list_node =
		arena.new_node(Node::new(Item::ParameterList(parameter_list), open_bracket.loc));
	let new_line = tokens.pop().unwrap();
	match new_line.token {
		Token::NewLine => {}
		_ => {
			panic!(
				"Expected a new line on line {}, column {}.",
				new_line.loc.line_num, new_line.loc.column_num
			);
		}
	}
	let block_node =
		arena.new_node(Node::new(Item::Block, Loc::new(new_line.loc.line_numn + 1, 0)));
	parse_block(1, &block_node, arena, tokens);
}

fn parse_block(
	indentation: i32,
	parent: &NodeId,
	arena: &mut Arena<Node>,
	tokens: &mut Vec<TokenValue>,
) {
	let mut finished = false;
	while !finished {
		for i in 1..indentation {
			let next = tokens.pop().unwrap();
			match next.token {
				Token::Tab => {}
				_ => finished = true,
			}
		}
		parse_statement(parent, arena, tokens)
	}
}

fn parse_statement(parent: &NodeId, arena: &mut Arena<Node>, tokens: &mut Vec<TokenValue>) {
	let lexitem = tokens.pop().unwrap();
	match lexitem.token {
		Token::Keyword(Keyword::If) => {
			let new_node = arena.new_node(Node::new(Item::If, lexitem.loc));
			parent.append(new_node, arena);
			parse_if(&new_node, arena, tokens);
		}
		Token::Keyword(Keyword::While) => {
			let new_node = arena.new_node(Node::new(Item::While, lexitem.loc));
			parent.append(new_node, arena);
			parse_while(&new_node, arena, tokens);
		}
		Token::Keyword(Keyword::For) => {
			let new_node = arena.new_node(Node::new(Item::For, lexitem.loc));
			parent.append(new_node, arena);
			parse_for(&new_node, arena, tokens)
		}
		Token::Keyword(Keyword::Function) => {
			let new_node = arena.new_node(Node::new(Item::Function, lexitem.loc));
			parent.append(new_node, arena);
			parse_function(&new_node, arena, tokens);
		}
		Token::Identifier(s) => {
			let next = tokens.pop().unwrap();
			match next.token {
				Token::Operator(Operator::Equals) => {
					let new_node = arena.new_node(Node::new(Item::Assign, lexitem.loc));
					parent.append(new_node, arena);
					let var_name = arena.new_node(Node::new(Item::Identifier(s), lexitem.loc));
					let assign_expression = arena.new_node(Node::new(Item::Expression, next.loc));
					new_node.append(var_name, arena);
					new_node.append(assign_expression, arena);
					parse_expression(&assign_expression, arena, tokens)
				}
				Token::NewLine => {}
				_ => {
					panic!(
						"Invalid token on line {}, column {}.",
						next.loc.line_num, next.loc.column_num
					);
				}
			}
		}
		_ => panic!(
			"Unexpected term on line {}, column {}.",
			lexitem.loc.line_num, lexitem.loc.column_num
		),
	}
}

pub fn lexer(tokens: &mut Vec<TokenValue>) -> &Arena<Node> {
	let arena = &mut Arena::new();
	let program_node = arena.new_node(Node::new(Item::Program, Loc::new(0, 0)));
	parse_statement(&program_node, arena, tokens);
	arena
}

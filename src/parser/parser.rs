use crate::parser::lexer::{Keyword, LiteralValue, Loc, Number, Operator, Token, TokenValue};
use indextree::{Arena, NodeId};

#[derive(Debug)]
#[derive(Clone)]
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

fn get_next_token(tokens: &mut Vec<TokenValue>) -> TokenValue {
    let result = tokens.first().unwrap().clone();
    tokens.remove(0);
    result
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Node {
    loc: Loc,
    item: Item,
}

impl Node {
    pub fn new(item: Item, loc: Loc) -> Self {
        Self { item, loc }
    }
}

fn parse_expression(indentation: i32, parent: &NodeId, arena: &mut Arena<Node>, tokens: &mut Vec<TokenValue>) {
    let mut output = Vec::new();
    let mut op_stack = Vec::new();
    loop {
        let next = tokens.remove(0);
        match &next.token {
            Token::Identifier(s) => {
                let look_ahead = tokens.iter().next().unwrap();
                match look_ahead.token {
                    Token::OpenBracket => op_stack.push(next),
                    _ => output.push(next),
                }
            }
            Token::Literal(LiteralValue::Number(_)) => {
                tokens.push(next);
            }
            Token::Operator(_) => {
                if priority(&next) > priority(op_stack.iter().next().unwrap()) {
                    op_stack.push(next);
                } else {
                    loop {
                        let next_operator = op_stack.remove(0);
                        if priority(&next_operator) >= priority(&next) {
                            output.push(next_operator);
                        } else {
                            op_stack.push(next);
                            break;
                        }
                    }
                }
            }
            Token::OpenBracket => op_stack.push(next),
            Token::CloseBracket => {
                while op_stack.get(0).unwrap().token != Token::CloseBracket {
                    output.push(match op_stack.get(0).unwrap().token {
                        Token::EndOfSequence => panic!(
                            "Missing a closing bracket on line {}, column {}",
                            op_stack.get(0).unwrap().loc.line_num,
                            op_stack.get(0).unwrap().loc.column_num
                        ),
                        _ => op_stack.remove(0),
                    })
                }
                op_stack.pop();
            }
            Token::EndOfSequence => {
                for i in 1..op_stack.clone().iter().count() {
                    output.push(op_stack.remove(0))
                }
            }
            _ => panic!(
                "Invalid token in the expression on line {}, column {}",
                next.loc.line_num, next.loc.column_num
            ),
        }
    }
    let mut stack: Vec<NodeId> = Vec::new();
    let mut last_operator = 0;
    for (pos, token) in output.clone().iter().enumerate() {
        if !is_operator(&token) {
            stack.push(arena.new_node(Node::new(
                match token.token {
                    Token::Identifier(s) => Item::Identifier(s),
                    Token::Literal(LiteralValue::Number(n)) => Item::Number(n),
                    _ => panic!(
                        "Invalid token in the expression on line {}, column {}",
                        token.loc.line_num, token.loc.column_num
                    ),
                },
                token.loc,
            )));
            last_operator += 1;
        } else {
            match token.token {
                Token::FunctionCall(s) => {
                    let function_node = arena.new_node(Node::new(Item::Call(s), token.loc));
                    for i in 1..last_operator {
                        function_node.append(stack.pop().unwrap(), arena)
                    }
                    stack.push(function_node);
                }
                Token::Operator(o) => {
                    let o1 = stack.pop().unwrap();
                    let o2 = stack.pop().unwrap();
                    let operator_node = arena.new_node(Node::new(Item::Operator(o), token.loc));
                    operator_node.append(o1, arena);
                    operator_node.append(o2, arena);
                    stack.push(operator_node);
                }
                _ => panic!(
                    "Invalid token in the expression on line {}, column {}",
                    token.loc.line_num, token.loc.column_num
                ),
            }
            last_operator = 0;
        }
    }
}

fn parse_if(indentation: i32, parent: &NodeId, arena: &mut Arena<Node>, tokens: &mut Vec<TokenValue>) {
    let mut then_token = false;
    let mut expression: Vec<TokenValue> = Vec::new();
    let start_loc = tokens.iter().next().unwrap().clone();
    while !then_token {
        let next = tokens.remove(0);
        match next.token {
            Token::Keyword(Keyword::Then) => {
                then_token = true;
                let new_line = tokens.remove(0);
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
    parse_expression(indentation, &expression_node, arena, &mut expression);
}

fn parse_while(indentation: i32, parent: &NodeId, arena: &mut Arena<Node>, tokens: &mut Vec<TokenValue>) {
    let n = arena.new_node(Node::new(Item::While, arena.get(*parent).unwrap().get().loc));
    parent.append(n, arena);
    let mut do_token_found = false;
    let mut expression: Vec<TokenValue> = Vec::new();
    while !do_token_found {
        let do_token = tokens.remove(0);
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
    parse_expression(indentation, &n, arena, &mut expression);
    let end_token = tokens.remove(0);
    if end_token.token != Token::Keyword(Keyword::EndWhile) {
        panic!(
            "Expected the keyword 'endwhile' on line {}, column {}",
            end_token.loc.line_num, end_token.loc.column_num
        )
    }
}

fn parse_for(indentation: i32, parent: &NodeId, arena: &mut Arena<Node>, tokens: &mut Vec<TokenValue>) {
    let for_node = arena.new_node(Node::new(Item::For, arena.get(*parent).unwrap().get().loc));
    parent.append(for_node, arena);

    let mut count_variable = String::new();
    let identifier = tokens.remove(0);
    match identifier.token {
        Token::Identifier(s) => {
            count_variable = s;
        }
        _ => panic!(
            "Expected a variable after the keyword 'for' on line {}, column {}",
            identifier.loc.line_num, identifier.loc.column_num
        ),
    }

    let equals_sign = tokens.remove(0);
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
            let next_token = tokens.remove(0);
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
            let next_token = tokens.remove(0);
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
        parse_expression(indentation, &for_node, arena, &mut expression_block_1);
        parse_expression(indentation, &for_node, arena, &mut expression_block_2);
        parse_block(1, &for_node, arena, tokens)
    } else {
        let mut expression: Vec<TokenValue> = Vec::new();
        let mut found_do = false;
        while !found_do {
            let next_token = tokens.remove(0);
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
        parse_expression(indentation, &for_node, arena, &mut expression);
        parse_block(1, &for_node, arena, tokens)
    }
}

fn parse_function(indentation: i32, parent: &NodeId, arena: &mut Arena<Node>, tokens: &mut Vec<TokenValue>) {
    let function_node =
        arena.new_node(Node::new(Item::Function, tokens.iter().next().unwrap().loc));
    parent.append(function_node, arena);
    let identifier_token = tokens.remove(0);
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
    let open_bracket = tokens.remove(0);
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
        let next_token = tokens.remove(0);
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
    let new_line = tokens.remove(0);
    match new_line.token {
        Token::NewLine => {}
        _ => {
            panic!(
                "Expected a new line on line {}, column {}.",
                new_line.loc.line_num, new_line.loc.column_num
            );
        }
    }
    let block_node = arena.new_node(Node::new(Item::Block, Loc::new(new_line.loc.line_num + 1, 0)));
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
            let next = tokens.remove(0);
            match next.token {
                Token::Tab => {}
                _ => finished = true,
            }
        }
        parse_statement(indentation, parent, arena, tokens)
    }
}

fn parse_statement(indentation: i32, parent: &NodeId, arena: &mut Arena<Node>, tokens: &mut Vec<TokenValue>) {
    let lexitem = tokens.remove(0);
    match lexitem.token {
        Token::Keyword(Keyword::If) => {
            let new_node = arena.new_node(Node::new(Item::If, lexitem.loc));
            parent.append(new_node, arena);
            parse_if(indentation, &new_node, arena, tokens);
        }
        Token::Keyword(Keyword::While) => {
            let new_node = arena.new_node(Node::new(Item::While, lexitem.loc));
            parent.append(new_node, arena);
            parse_while(indentation, &new_node, arena, tokens);
        }
        Token::Keyword(Keyword::For) => {
            let new_node = arena.new_node(Node::new(Item::For, lexitem.loc));
            parent.append(new_node, arena);
            parse_for(indentation, &new_node, arena, tokens)
        }
        Token::Keyword(Keyword::Function) => {
            let new_node = arena.new_node(Node::new(Item::Function, lexitem.loc));
            parent.append(new_node, arena);
            parse_function(indentation, &new_node, arena, tokens);
        }
        Token::Identifier(s) => {
            let next = tokens.remove(0);
            match next.token {
                Token::Operator(Operator::Equals) => {
                    let assign_node = arena.new_node(Node::new(Item::Assign, lexitem.loc));
                    parent.append(assign_node, arena);

                    let var_name = arena.new_node(Node::new(Item::Identifier(s), lexitem.loc));
                    let assign_expression = arena.new_node(Node::new(Item::Expression, next.loc));

                    assign_node.append(var_name, arena);
                    assign_node.append(assign_expression, arena);

                    parse_expression(indentation, &assign_expression, arena, tokens)
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
        Token::EndOfSequence => {}
        _ => panic!(
            "Unexpected token on line {}, column {}.",
            lexitem.loc.line_num, lexitem.loc.column_num
        ),
    }
}

pub fn parser(tokens: &mut Vec<TokenValue>) -> (Arena<Node>, NodeId) {
    let arena = &mut Arena::new();
    let program_node = arena.new_node(Node::new(Item::Program, Loc::new(0, 0)));
    parse_statement(0, &program_node, arena, tokens);
    (arena.clone(), program_node)
}

use crate::parser::lexer::{Keyword, LiteralValue, Loc, Number, Operator, Token, TokenValue};
use indextree::{Arena, NodeId};

pub enum Item {
    Function,
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
}

pub fn priority(o: &Operator) -> u32 {
    match o {
        Operator::Times => 5,
        Operator::Divide => 5,
        Operator::IntegerDivide => 5,
        Operator::Plus => 4,
        Operator::Minus => 4,
        Operator::Equals => 3,
        Operator::Modulo => 5,
        Operator::And => 2,
        Operator::Or => 1,
        Operator::Not => 5,
        // This is kept here in case more operators are to be added
        _ => 0,
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

fn parse_boolean() {}

fn parse_arithmetic() {}

fn parse_assignment() {}

/// E -> E "+" E | E "-" E | E "/" E | E "//" E | E "*" E | E "or" E | E "and" E | "not" E | N
/// N
/// This is an implementation of the Shunting-Yard algorithm.
fn parse_expression(parent: &NodeId, arena: &mut Arena<Node>, tokens: &mut Vec<TokenValue>) {
    let mut operator_stack: Vec<Operator> = Vec::new();
    let mut operand_stack: Vec<TokenValue> = Vec::new();
    let mut finished = false;
    while !finished {
        let next: &TokenValue = tokens.iter().next().unwrap();
        match &next.token {
            Token::Operator(o) => {
                let t = tokens.pop().unwrap();
                match t.token {
                    Token::Operator(o) => {
                        if priority(&o) > priority(operator_stack.iter().next().unwrap()) {
                            operator_stack.push(o);
                        } else {
                            let mut lower = false;
                            while !lower {
                                let o_1 = operand_stack.pop().unwrap();
                                let o_2 = operand_stack.pop().unwrap();
                                let n_1 = arena.new_node(match o_1.token {
                                    Token::Literal(LiteralValue::Number(n)) => {
                                        Node::new(Item::Number(n), o_1.loc)
                                    }
                                    Token::Literal(LiteralValue::String(s)) => {
                                        Node::new(Item::Identifier(s), o_1.loc)
                                    }
                                    _ => panic!(
                                        "Invalid token on line {} column {}",
                                        o_1.loc.line_num, o_1.loc.column_num
                                    ),
                                });
                                let n_2 = arena.new_node(match o_2.token {
                                    Token::Literal(LiteralValue::Number(n)) => {
                                        Node::new(Item::Number(n), o_2.loc)
                                    }
                                    Token::Literal(LiteralValue::String(s)) => {
                                        Node::new(Item::Identifier(s), o_2.loc)
                                    }
                                    _ => panic!(
                                        "Invalid token on line {} column {}",
                                        o_2.loc.line_num, o_2.loc.column_num
                                    ),
                                });
                            }
                        }
                    }
                    Token::Identifier(_) => {
                        operand_stack.push(t.clone());
                    }
                    Token::Literal(LiteralValue::Number(_)) => operand_stack.push(t.clone()),
                    _ => {}
                }
            }
            Token::Identifier(_) | Token::Literal(_) => {}
            _ => {
                finished = true;
            }
        }
    }
}

fn parse_if(parent: &NodeId, arena: &mut Arena<Node>, tokens: &mut Vec<TokenValue>) {
    let expression = arena.new_node(Node::new(
        Item::Expression,
        tokens.iter().next().unwrap().loc,
    ));
    parent.append(expression, arena);
    parse_expression(&expression, arena, tokens);
}

fn parse_while() {}

fn parse_for() {}

fn parse_function() {}

fn parse_statement(parent: &NodeId, arena: &mut Arena<Node>, tokens: &mut Vec<TokenValue>) {
    let mut lexitem = tokens.pop().unwrap();
    match lexitem.token {
        Token::Keyword(Keyword::If) => {
            let new_node = arena.new_node(Node::new(Item::If, lexitem.loc));
            parent.append(new_node, arena);
            parse_if(&new_node, arena, tokens);
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

enum TokenValue {
    Identifier(String),
    Keyword(String),
    Separator(String),
    Operator(String),
    Literal(String),
    Comment(String),
}

pub fn lexer(input: &String) {
    // Reverse direction of string
    let mut input_stack: String = String::from(input).chars().rev().collect();
    while input_stack.len() == 0 {
        let mut top = input_stack.pop().expect("Could not pop item");
        match top {
            // match identifier
            'a'..'Z' => {
                let mut finished = false;
                let mut identifier = top.to_string();
                top = input_stack.pop().expect("Could not get another token");
                while !finished {
                    match top {
                        'a'..'z' => {
                            identifier.push(top);
                        }
                        '0'..'9' => {
                            identifier.push(top)
                        }
                        _ => {
                            panic!("Invalid token {}", top)
                        }
                    }
                    if input_stack.len() != 0 {
                        top = input_stack.pop().expect("Could not retrieve the next token.")
                    } else {
                        finished = true;
                    }
                };
            }
            // match number
            '0'..'9' => {}
            _ => {
                panic!("Found an invalid token {}!", top)
            }
        };
    };
}

pub fn parser() {}
use crate::parser::lexer::{TokenValue, Token, Keyword};

pub fn to_js(tokens: &mut Vec<TokenValue>) -> String {
    let mut finished = false;
    let mut output = String::new();
    while !finished {
        let next = tokens.pop().unwrap();
        match next.token {
            Token::Keyword(k) => {
                match k {
                    Keyword::Function => {
                        let next = tokens.pop().unwrap();
                        if next.token != Token::OpenBracket {
                            panic!("Expected an opening bracket after a function declaration on line {}, column {}.", next.loc.line_num, next.loc.column_num)
                        }
                        let mut close_bracket = false;
                        while !close_bracket {
                            let mut close_token = tokens.pop().unwrap();
                            match close_token.token {
                                Token::CloseBracket => {
                                    close_bracket = true;
                                }
                                Token::Identifier(_) => {}
                                Token::Comma => {}
                                Token::EndOfSequence => {
                                    panic!("Missing close bracket after opening bracket in the function defenition on line {}, column {}.", next.loc.line_num, next.loc.column_num)
                                }
                                _ => {
                                    panic!("Unexpected token on line {}, column {}!", close_token.loc.line_num, close_token.loc.column_num)
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
    output
}
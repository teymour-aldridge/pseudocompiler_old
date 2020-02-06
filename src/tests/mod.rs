#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use crate::parser::*;
        assert_eq!(lexer(&String::from("hello"))[0], TokenValue::Identifier(String::from("hello"), Loc::new(1, 5)))
    }
}

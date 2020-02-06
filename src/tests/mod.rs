#[cfg(test)]
mod tests {
    #[test]
    fn assignment() {
        use crate::parser::*;
        println!("{:?}", lexer(&String::from("hello =")))
    }
}

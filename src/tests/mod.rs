#[cfg(test)]
mod tests {
    #[test]
    fn assignment() {
        use crate::parser::*;
        println!("{:?}", lexer(&String::from("1.6e8")))
    }
}

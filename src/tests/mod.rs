#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use crate::parser::lexer;
        println!(lexer(&String::from("hello")))
    }
}

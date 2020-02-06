#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use crate::parser::*;
        println!("{:?}", lexer(&String::from("hello")))
    }
}

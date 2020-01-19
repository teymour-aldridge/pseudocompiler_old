#[cfg(test)]
mod tests {
    use crate::tokeniser::tokenise_line;

    #[test]
    fn tokeniser() {
        assert_eq!(tokenise_line(String::from("for i in range(12)")))
    }
}

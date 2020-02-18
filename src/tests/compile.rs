use crate::compile;
#[test]
fn test_compiler() {
    println!("{}", compile(String::from("x=1")))
}
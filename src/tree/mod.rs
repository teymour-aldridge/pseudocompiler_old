use std::path::Component::ParentDir;
use crate::tokeniser::Token;

enum Parent<T> {
    Left(Box<Option<Node<T>>>),
    Right(Box<Option<Node<T>>>),
    None,
}

pub struct Node<T> {
    data: T,
    left: Box<Option<Node<T>>>,
    right: Box<Option<Node<T>>>,
    parent: Box<Parent<T>>,
}


pub fn tree(mut tokens: Vec<Token>) {
    for token in tokens {}
}
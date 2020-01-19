use std::path::Component::ParentDir;

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

impl<T> Node<T> {
    fn new(data: T, left: Option<Node<T>>, right: Option<Node<T>>, parent: Parent<T>) -> Self {
        Self {
            data,
            left: Box::new(left),
            right: Box::new(right),
            parent: Box::new(parent),
        }
    }
    fn left_child(&mut self) -> &mut Option<Node<T>> {
        return &mut self.left;
    }
    fn right_child(&mut self) -> &mut Option<Node<T>> {
        return &mut self.right;
    }
    fn sibling(&mut self) -> &mut Parent<T> {
        match &mut self.parent {
            Some(parent) => {
                match parent {
                    Parent::Left(mut parent) => &mut parent.right,
                    Parent::Right(mut parent) => &mut parent.left,
                    Parent::None => &mut Parent::None
                }
            }
            None => None
        }
    }
    fn parent(&mut self) -> &mut Option<Node<T>> {
        match &mut self.parent {
            Some(mut parent) => {
                match parent {
                    Parent::Left(mut parent) => &parent,
                    Parent::Right(mut parent) => &parent,
                    Parent::None => None
                }
            }
            None => None
        }
    }
}
use std::path::Component::ParentDir;

enum Parent {
    Left(Option<Node<T>>),
    Right(Option<Node<T>>),
    None,
}

pub struct Node<T> {
    data: T,
    left: Option<Node<T>>,
    right: Option<Node<T>>,
    parent: Parent,
}

impl<T> Node<T> {
    fn new(data: T, left: Option<Node<T>>, right: Option<Node<T>>, parent: Parent) -> Self {
        Self {
            data,
            left,
            right,
            parent,
        }
    }
    fn left_child(&mut self) -> &mut Option<Node<T>> {
        return &mut self.left;
    }
    fn right_child(&mut self) -> &mut Option<Node<T>> {
        return &mut self.right;
    }
    fn sibling(&mut self) -> &mut Parent {
        match self.parent {
            Parent::Left => &mut self.parent.right,
            Parent::Right => &mut self.parent.left,
            Parent::None => &mut Parent::None
        }
    }
    fn parent(&mut self) -> &mut Option<Node<T>> {
        match self.parent {
            Parent::Left(mut parent) => &parent,
            Parent::Right(mut parent) => &parent,
            Parent::None => None
        }
    }
}
pub enum TreeNode<T> {
    SubNode(BTree<T>),
    Data(T),
    None,
}

pub struct BTree<T> {
    data: T,
    left: Option<T>,
    right: Option<T>,
}

impl BTree<T> {
    pub fn new<T>(d: T, l: Option<T>, r: Option<t>) -> Self {
        Self {
            data: d,
            left: l,
            right: r,
        }
    }
}

impl BTree<TreeNode<T>> {}

pub fn parser() {}
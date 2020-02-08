use super::lexer::Loc;
use crate::parser::lexer::Operator;

pub enum ASTItem {
    Sequence(Loc),
    If(Loc),
    ElseIf(Loc),
    While(Loc),
    Return(Loc),
    Compare(Operator, Loc),
    Assign(Loc),
    Variable(String, Loc),
    Operate(Operator, Loc),
    Function(String, Loc),
}

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
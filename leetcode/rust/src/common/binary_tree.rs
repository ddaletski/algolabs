use std::cell::RefCell;
use std::rc::Rc;

pub type NodeLink<T> = Option<Rc<RefCell<TreeNode<T>>>>;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode<T> {
    pub val: T,
    pub left: NodeLink<T>,
    pub right: NodeLink<T>,
}

impl<T> TreeNode<T> {
    #[inline]
    pub fn new(val: T) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    #[inline]
    pub fn new_link(val: T, left: NodeLink<T>, right: NodeLink<T>) -> NodeLink<T> {
        Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
    }
}

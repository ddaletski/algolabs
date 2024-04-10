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

    pub fn from_vec_dfs(vec: Vec<Option<T>>) -> NodeLink<T> {
        let mut vec = vec;
        Self::dfs(&mut vec)
    }

    fn dfs(vec: &mut Vec<Option<T>>) -> NodeLink<T> {
        if vec.is_empty() {
            None
        } else {
            let val = vec.remove(0)?;
            let left = Self::dfs(vec);
            let right = Self::dfs(vec);
            Self::new_link(val, left, right)
        }
    }
}

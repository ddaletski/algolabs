struct Solution;

type TreeNode = crate::common::binary_tree::TreeNode<i32>;

//////////////////////////////////////////////////////////

type NodeLink = Option<Rc<RefCell<TreeNode>>>;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn max_path_sum(root: NodeLink) -> i32 {
        let mut max_sum = i32::MIN / 3;

        Self::dfs(root, &mut max_sum);

        max_sum
    }

    fn dfs(root: NodeLink, max_sum: &mut i32) -> i32 {
        let Some(node) = root else {
            return 0;
        };
        let node = node.borrow();

        let val = node.val;
        let left_sum = Self::dfs(node.left.clone(), max_sum);
        let right_sum = Self::dfs(node.right.clone(), max_sum);

        let with_split = left_sum + right_sum + val;
        let without_split = left_sum.max(right_sum) + val;

        *max_sum = (*max_sum).max(with_split);
        *max_sum = (*max_sum).max(without_split);

        without_split.max(0)
    }
}

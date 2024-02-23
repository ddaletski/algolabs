type TreeNode = crate::common::binary_tree::TreeNode<i32>;
pub struct Solution {}

/////////////////////////////////////////////////////////////////

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

type NodeLink = Option<Rc<RefCell<TreeNode>>>;
impl Solution {
    pub fn deepest_leaves_sum(root: NodeLink) -> i32 {
        let mut queue: VecDeque<(u32, Rc<RefCell<TreeNode>>)> = VecDeque::new();

        if root.is_none() {
            return 0;
        }

        queue.push_back((0, root.unwrap()));

        let mut last_level = 0;
        let mut level_sum = 0;

        while !queue.is_empty() {
            let (level, node) = queue.pop_front().unwrap();

            if level > last_level {
                level_sum = 0;
                last_level += 1;
            }

            let node_ref = node.borrow();

            level_sum += node_ref.val;

            if let Some(left) = node_ref.left.clone() {
                queue.push_back((level + 1, left));
            }
            if let Some(right) = node_ref.right.clone() {
                queue.push_back((level + 1, right));
            }
        }

        level_sum
    }
}

/////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_returns;

    #[test]
    fn it_works() {
        let tree = TreeNode::new_link(
            1,
            TreeNode::new_link(
                2,
                TreeNode::new_link(4, TreeNode::new_link(7, None, None), None),
                TreeNode::new_link(5, None, None),
            ),
            TreeNode::new_link(
                3,
                None,
                TreeNode::new_link(6, None, TreeNode::new_link(8, None, None)),
            ),
        );

        assert_returns!(15, Solution::deepest_leaves_sum, tree);
    }
}

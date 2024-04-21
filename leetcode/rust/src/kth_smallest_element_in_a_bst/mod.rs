use crate::common;

type NodeLink = common::binary_tree::NodeLink<i32>;
struct Solution;

/////////////////////////////////////////////////////

impl Solution {
    pub fn kth_smallest(root: NodeLink, k: i32) -> i32 {
        let k = k as usize;
        fn inorder(link: NodeLink, buf: &mut Vec<i32>, k: usize) {
            let Some(node_ref) = link else {
                return;
            };

            inorder(node_ref.borrow().left.clone(), buf, k);
            if buf.len() == k {
                return;
            }
            buf.push(node_ref.borrow().val);
            inorder(node_ref.borrow().right.clone(), buf, k);
        }

        let mut buf = Vec::with_capacity(k);
        inorder(root, &mut buf, k);

        buf[k - 1]
    }
}
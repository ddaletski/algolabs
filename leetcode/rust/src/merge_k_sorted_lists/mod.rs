type ListNode = crate::common::linked_list::ListNode<i32>;
pub struct Solution {}

//////////////////////////////////////////////////////////////

use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Eq, PartialEq)]
struct HeapItem(ListNode);

impl Ord for HeapItem {
    fn cmp(&self, other: &Self) -> Ordering {
        other.0.val.cmp(&self.0.val)
    }
}
impl PartialOrd for HeapItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl From<ListNode> for HeapItem {
    fn from(node: ListNode) -> Self {
        Self(node)
    }
}

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut heap = BinaryHeap::with_capacity(lists.len());

        for list in lists {
            if let Some(node) = list {
                heap.push(HeapItem(*node));
            }
        }

        let mut head: Box<ListNode> = Box::new(ListNode::new(0));
        let mut tail = &mut head;

        while !heap.is_empty() {
            let HeapItem(next_node) = heap.pop().unwrap();

            tail.next = Some(Box::new(ListNode::new(next_node.val)));
            tail = tail.next.as_mut().unwrap();

            if let Some(child) = next_node.next {
                heap.push(HeapItem(*child));
            }
        }

        head.next
    }
}

////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::linked_list;

    #[test]
    fn case1() {
        let input_values = vec![vec![1, 4, 5], vec![1, 3, 4], vec![2, 6]];
        let expected_values = vec![1, 1, 2, 3, 4, 4, 5, 6];

        let input_list: Vec<_> = input_values
            .into_iter()
            .map(|vals| linked_list::vec_to_list(vals))
            .collect();

        let result_list = Solution::merge_k_lists(input_list);

        let result_values = linked_list::list_to_vec(result_list);

        assert_eq!(result_values, expected_values);
    }

    #[test]
    fn empty_input() {
        let empty_input = vec![None];
        let result = Solution::merge_k_lists(empty_input);

        assert_eq!(result, None);
    }
}

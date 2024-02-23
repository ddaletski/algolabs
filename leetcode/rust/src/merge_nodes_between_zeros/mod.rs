type ListNode = crate::common::linked_list::ListNode<i32>;
pub struct Solution {}

/////////////////////////////////////////////////////

impl Solution {
    pub fn merge_nodes(mut input_head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        input_head = input_head.unwrap().next;

        let mut current_sum = 0;

        let mut result_head = Box::new(ListNode::new(0));
        let mut result_tail = &mut result_head;

        while let Some(node) = input_head {
            current_sum += node.val;
            if node.val == 0 {
                result_tail.next = Some(Box::new(ListNode::new(current_sum)));
                result_tail = result_tail.next.as_mut().unwrap();
                current_sum = 0;
            }
            input_head = node.next;
        }

        result_head.next
    }
}

////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::linked_list::{list_to_vec, vec_to_list};
    use rstest::rstest;

    #[rstest]
    #[case(vec![0,3,1,0,4,5,2,0],vec![4,11])]
    #[case(vec![0,1,0,3,0,2,2,0], vec![1,3,4])]
    fn simple_tests(#[case] input: Vec<i32>, #[case] expected: Vec<i32>) {
        let input_list = vec_to_list(input);
        let output_list = Solution::merge_nodes(input_list);
        let output = list_to_vec(output_list);

        assert_eq!(output, expected);
    }
}

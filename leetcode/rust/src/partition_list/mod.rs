type ListNode = crate::common::linked_list::ListNode<i32>;
pub struct Solution;

///////////////////////////////////////////////////

impl Solution {
    pub fn partition(mut head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        let mut head1 = ListNode::new(0);
        let mut tail1 = &mut head1;

        let mut head2 = ListNode::new(0);
        let mut tail2 = &mut head2;

        while let Some(mut head_) = head {
            head = std::mem::replace(&mut head_.next, None);

            if head_.val < x {
                tail1.next = Some(head_);
                tail1 = tail1.next.as_mut().unwrap();
            } else {
                tail2.next = Some(head_);
                tail2 = tail2.next.as_mut().unwrap();
            }
        }

        // link second list to the first one
        tail1.next = head2.next;

        head1.next
    }
}

///////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use crate::common::linked_list;

    #[rstest]
    #[case(vec![1,4,3,2,5,2], 3, vec![1,2,2,4,3,5])]
    #[case(vec![2, 1], 2, vec![1, 2])]
    #[case(vec![], 0, vec![])]
    #[case(vec![1], 0, vec![1])]
    #[case(vec![1], 2, vec![1])]
    #[test]
    fn it_works(#[case] values: Vec<i32>, #[case] pivot: i32, #[case] expected_result: Vec<i32>) {
        let input_list = linked_list::vec_to_list(values);
        let result_list = Solution::partition(input_list, pivot);
        let result = linked_list::list_to_vec(result_list);
        assert_eq!(result, expected_result);
    }
}

type ListNode = crate::common::linked_list::ListNode<i32>;
struct Solution;

////////////////////////////////////////////////////////////

impl Solution {
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let Some(mut head) = head else {
            return None;
        };

        let Some(mut next) = head.next.take() else {
            return Some(head);
        };

        let next_next = next.next.take();

        head.next = Self::swap_pairs(next_next);
        next.next = Some(head);

        Some(next)
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;
    use crate::common::linked_list::IntoVec;

    #[rstest]
    #[case(vec![1, 2, 3, 4], vec![2, 1, 4, 3])]
    #[case(vec![1], vec![1])]
    #[case(vec![1, 2], vec![2, 1])]
    fn test(#[case] input: Vec<i32>, #[case] expected: Vec<i32>) {
        let input = ListNode::from_vec(input);
        let output = Solution::swap_pairs(input).into_vec();

        assert_eq!(output, expected);
    }
}

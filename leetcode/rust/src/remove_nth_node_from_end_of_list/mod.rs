struct Solution;

type ListNode = crate::common::linked_list::ListNode<i32>;

impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut head = Some(Box::new(ListNode {
            val: -1,
            next: head,
        }));

        let mut fast = &head;

        let mut fast_distance = 0;
        while let Some(node) = fast {
            fast = &node.next;
            fast_distance += 1;
            if fast_distance == n + 1 {
                break;
            }
        }

        let fast = fast as *const Option<Box<ListNode>>;
        let mut slow = &mut head as *mut Option<Box<ListNode>>;
        let mut fast = unsafe { fast.as_ref() }.unwrap();

        while let Some(node) = fast {
            fast = &node.next;
            let slow_ref = unsafe { slow.as_mut().unwrap() };
            let next = &mut slow_ref.as_mut().unwrap().next;
            slow = next as *mut _;
        }

        let before_removed = unsafe { slow.as_mut().unwrap() }.as_mut().unwrap();
        let mut removed = before_removed.next.take().unwrap();
        let after_removed = removed.next.take();

        before_removed.as_mut().next = after_removed;

        head.unwrap().next
    }
}

#[cfg(test)]
mod tests {
    use crate::common::linked_list::{list_to_vec, vec_to_list};

    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(vec![1, 2, 3, 4, 5], 2, vec![1, 2, 3, 5])]
    #[case(vec![1], 1, vec![])]
    #[case(vec![1, 2], 1, vec![1])]
    fn test(#[case] input: Vec<i32>, #[case] n: i32, #[case] expected: Vec<i32>) {
        let head = vec_to_list(input);
        let result = Solution::remove_nth_from_end(head, n);
        let result = list_to_vec(result);
        assert_eq!(result, expected);
    }
}

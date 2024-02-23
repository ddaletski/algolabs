type ListNode = crate::common::linked_list::ListNode<i32>;
pub struct Solution;

/////////////////////////////////////////////////////////

impl Solution {
    pub fn delete_middle(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return None;
        }

        unsafe {
            let mut ptr1: *mut Box<ListNode> = head.as_mut().unwrap();
            if (*ptr1).next.is_none() {
                return None;
            }
            let mut ptr2: *const Box<ListNode> = (*ptr1).next.as_ref().unwrap();

            while let Some(next) = (*ptr2).next.as_ref() {
                if let Some(next_next) = next.next.as_ref() {
                    ptr2 = next_next;
                } else {
                    break;
                }

                ptr1 = (*ptr1).next.as_mut().unwrap();
            }

            let ptr_mid: *mut Box<ListNode> = (*ptr1).next.as_mut().unwrap();
            let ptr_after_mid = std::mem::replace(&mut (*ptr_mid).next, None);
            (*ptr1).next = ptr_after_mid;
        }

        head
    }
}

//////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use crate::common::linked_list;

    use super::*;
    use proptest::{prop_assert_eq, proptest};

    #[test]
    fn empty() {
        let list = None;
        let result = Solution::delete_middle(list);
        assert_eq!(result, None);
    }

    proptest! {
        #[test]
        fn test_random_vecs(vals in proptest::collection::vec(-100..100, 1..30)) {
            let mut expected_result = vals.clone();
            expected_result.remove(vals.len() / 2);

            let list = linked_list::vec_to_list(vals);
            let list_removed = Solution::delete_middle(list);
            let vals_removed = linked_list::list_to_vec(list_removed);

            prop_assert_eq!(vals_removed, expected_result);
        }
    }
}

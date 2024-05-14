struct Solution;

///////////////////////////////////////////

use std::cmp::Ordering;

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        Self::bin_search(&nums, 0, nums.len(), target) as i32
    }

    fn bin_search(nums: &Vec<i32>, left: usize, right: usize, target: i32) -> usize {
        if left == right {
            return left;
        }

        let mid = (left + right) / 2;
        let mid_item = nums[mid];

        match target.cmp(&mid_item) {
            Ordering::Less => Self::bin_search(nums, left, mid, target),
            Ordering::Greater => Self::bin_search(nums, mid + 1, right, target),
            _ => mid,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use algo_toolbox::assert_returns;
    use rstest::rstest;

    #[rstest]
    #[case(vec![1, 3, 5, 6], 5, 2)]
    #[case(vec![1, 3, 5, 6], 2, 1)]
    #[case(vec![1, 3, 5, 6], 7, 4)]
    #[case(vec![1, 3, 5, 6], 0, 0)]
    fn test_search_insert(#[case] nums: Vec<i32>, #[case] target: i32, #[case] expected: i32) {
        assert_returns!(expected, Solution::search_insert, nums, target);
    }
}

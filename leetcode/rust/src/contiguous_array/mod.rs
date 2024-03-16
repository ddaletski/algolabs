use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn find_max_length(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let mut diff_pos = HashMap::new();
        diff_pos.insert(0, 0);

        let mut one_zero_diff: i16 = 0;
        let mut max_len: u16 = 0;

        for (i, num) in nums.into_iter().enumerate() {
            if num == 0 {
                one_zero_diff -= 1;
            } else {
                one_zero_diff += 1;
            }

            if let Some(&prev_idx) = diff_pos.get(&one_zero_diff) {
                max_len = max_len.max(i as u16 + 1 - prev_idx);
            } else {
                diff_pos.insert(one_zero_diff, i as u16 + 1);
            }
        }

        max_len as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_returns;
    use rstest::rstest;

    #[rstest]
    #[case(vec![0, 1], 2)]
    #[case(vec![0, 1, 0], 2)]
    #[case(vec![0, 0, 1, 1, 0, 0, 1, 0], 6)]
    fn cases(#[case] nums: Vec<i32>, #[case] expected_result: i32) {
        assert_returns!(expected_result, Solution::find_max_length, nums);
    }
}

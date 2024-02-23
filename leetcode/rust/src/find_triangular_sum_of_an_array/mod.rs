pub struct Solution;

impl Solution {
    pub fn triangular_sum(mut nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let mut end = nums.len() - 1;

        while end != 0 {
            end -= 1;
            for i in 0..=end {
                nums[i] = (nums[i] + nums[i + 1]) % 10;
            }
        }

        nums[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(vec![1, 2, 3, 4, 5], 8)]
    #[case(vec![5, 4, 3, 2, 1], 8)]
    #[case(vec![5, 2, 3], 2)]
    #[case(vec![3, 2, 5], 2)]
    fn it_works(#[case] nums: Vec<i32>, #[case] expected_result: i32) {
        let result = Solution::triangular_sum(nums);
        assert_eq!(result, expected_result);
    }
}

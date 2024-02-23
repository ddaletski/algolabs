pub struct Solution {}

impl Solution {
    pub fn jump(mut nums: Vec<i32>) -> i32 {
        let mut distances = vec![];
        distances.resize(nums.len(), nums.len() as i32);

        Solution::jump_impl(&mut nums, &mut distances, 0, 0)
    }

    fn jump_impl(nums: &Vec<i32>, distances: &mut Vec<i32>, current_idx: usize, jumps: i32) -> i32 {
        if current_idx >= nums.len() - 1 {
            return jumps;
        }

        if jumps >= distances[current_idx] {
            return -1;
        }
        distances[current_idx] = jumps;

        let max_distance = nums[current_idx] as usize;

        let mut results = vec![];
        for jump in (1..=max_distance).rev() {
            let result = Solution::jump_impl(nums, distances, current_idx + jump, jumps + 1);
            if result > 0 {
                results.push(result);
            }
        }

        results.into_iter().min().unwrap_or(-1)
    }
}

#[cfg(test)]
mod tests {
    use crate::assert_returns;

    use super::*;

    #[test]
    fn case1() {
        let nums = vec![2, 3, 1, 1, 4];
        let result = 2;

        assert_returns!(result, Solution::jump, nums.clone());
    }

    #[test]
    fn case2() {
        let nums = vec![2, 3, 0, 1, 4];
        let result = 2;

        assert_returns!(result, Solution::jump, nums.clone());
    }

    #[test]
    fn case3() {
        let nums = vec![
            5, 6, 4, 4, 6, 9, 4, 4, 7, 4, 4, 8, 2, 6, 8, 1, 5, 9, 6, 5, 2, 7, 9, 7, 9, 6, 9, 4, 1,
            6, 8, 8, 4, 4, 2, 0, 3, 8, 5,
        ];
        let result = 5;

        assert_returns!(result, Solution::jump, nums.clone());
    }

    #[test]
    fn case4() {
        let nums = vec![9, 8, 2, 2, 0, 2, 2, 0, 4, 1, 5, 7, 9, 6, 6, 0, 6, 5, 0, 5];
        let result = 3;
        assert_returns!(result, Solution::jump, nums.clone());
    }
}

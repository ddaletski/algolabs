pub struct Solution;

///////////////////

impl Solution {
    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        nums.sort();

        let mut closest_sum: i32 = nums.iter().take(3).sum();

        for (i, &num1) in nums.iter().take(nums.len() - 2).enumerate() {
            let mut left = i + 1;
            let mut right = nums.len() - 1;

            while left < right {
                let sum = num1 + nums[left] + nums[right];
                if (sum - target).abs() < (closest_sum - target).abs() {
                    closest_sum = sum;
                }

                if sum < target {
                    left += 1;
                } else if sum > target {
                    right -= 1;
                } else {
                    return target;
                }
            }
        }

        closest_sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_returns;

    #[test]
    fn case1() {
        let nums = vec![-1, 2, 1, -4];
        assert_returns!(2, Solution::three_sum_closest, nums, 1);
    }

    #[test]
    fn case2() {
        let nums = vec![0, 0, 0];
        assert_returns!(0, Solution::three_sum_closest, nums, 1);
    }
}

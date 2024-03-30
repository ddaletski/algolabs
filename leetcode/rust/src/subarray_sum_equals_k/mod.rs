struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut result = 0;

        let mut sums_count: HashMap<i32, i32> = HashMap::with_capacity(nums.len());
        sums_count.insert(0, 1);

        let mut cumsum = 0;
        for num in nums {
            cumsum += num;
            if let Some(&found_before) = sums_count.get(&(cumsum - k)) {
                result += found_before;
            }

            sums_count
                .entry(cumsum)
                .and_modify(|cnt| *cnt += 1)
                .or_insert(1);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use algo_toolbox::assert_returns;

    use super::*;

    #[test]
    fn test_1() {
        assert_returns!(2, Solution::subarray_sum, vec![1, 1, 1], 2);
    }

    #[test]
    fn test_2() {
        assert_returns!(2, Solution::subarray_sum, vec![1, 2, 3], 3);
    }
}

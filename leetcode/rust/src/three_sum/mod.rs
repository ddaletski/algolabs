use std::collections::{HashMap, HashSet};

pub struct Solution;

///////////////////

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        Solution::three_sum_2_optimized(nums)
    }

    pub fn three_sum_1(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort();

        let mapping: HashMap<i32, usize> = nums
            .iter()
            .cloned()
            .enumerate()
            .map(|(idx, value)| (value, idx))
            .collect();

        let mut result = HashSet::new();

        for (i, &num1) in nums.iter().enumerate() {
            for (j, &num2) in nums.iter().enumerate().skip(i + 1) {
                let candidate = -(num1 + num2);
                if let Some(&k) = mapping.get(&candidate) {
                    if k > j {
                        result.insert((num1, num2, candidate));
                    }
                }
            }
        }

        result.into_iter().map(|v| vec![v.0, v.1, v.2]).collect()
    }

    pub fn three_sum_2(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort();

        let mut result = HashSet::new();

        for (i, &num1) in nums.iter().take(nums.len() - 2).enumerate() {
            let mut left = i + 1;
            let mut right = nums.len() - 1;

            while left < right {
                let sum = num1 + nums[left] + nums[right];

                if sum < 0 {
                    left += 1;
                } else if sum > 0 {
                    right -= 1;
                } else {
                    result.insert((num1, nums[left], nums[right]));
                    left += 1;
                    right -= 1;
                }
            }
        }

        result.into_iter().map(|v| vec![v.0, v.1, v.2]).collect()
    }

    pub fn three_sum_2_optimized(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort_unstable();

        let mut result = vec![];

        for (i, &num1) in nums.iter().take(nums.len() - 2).enumerate() {
            if i > 0 && num1 == nums[i - 1] {
                continue;
            }
            if num1 > 0 {
                break;
            }

            let mut left = i + 1;
            let mut right = nums.len() - 1;

            while left < right {
                let sum = num1 + nums[left] + nums[right];

                if sum < 0 {
                    left += 1;
                } else if sum > 0 {
                    right -= 1;
                } else {
                    result.push(vec![num1, nums[left], nums[right]]);
                    left += 1;
                    right -= 1;
                    while left < right && nums[left] == nums[left - 1] {
                        left += 1;
                    }
                    while left < right && nums[right] == nums[right + 1] {
                        right -= 1;
                    }
                }

                if nums[right] < 0 {
                    break;
                }
            }
        }

        result
    }

    pub fn three_sum_3(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut negatives: HashMap<i32, i32> = HashMap::new();
        let mut positives: HashMap<i32, i32> = HashMap::new();
        let mut zeros = 0;

        for num in nums.into_iter() {
            if num < 0 {
                *negatives.entry(num).or_default() += 1;
            } else if num > 0 {
                *positives.entry(num).or_default() += 1;
            } else {
                zeros += 1;
            }
        }

        let mut result = HashSet::new();

        let mut add_to_result = |v1, v2, v3| {
            let mut vals = vec![v1, v2, v3];
            vals.sort();
            result.insert(vals)
        };

        if zeros > 2 {
            add_to_result(0, 0, 0);
        }

        // two negatives and one positive
        for (&neg_num1, &freq1) in &negatives {
            for &neg_num2 in negatives.keys() {
                if neg_num1 == neg_num2 && freq1 < 2 {
                    continue;
                }

                let candidate = -(neg_num1 + neg_num2);
                if positives.contains_key(&candidate) {
                    add_to_result(neg_num1, neg_num2, candidate);
                }
            }
        }

        // two positives and one negative
        for (&pos_num1, &freq1) in &positives {
            for &pos_num2 in positives.keys() {
                if pos_num1 == pos_num2 && freq1 < 2 {
                    continue;
                }

                let candidate = -(pos_num1 + pos_num2);
                if negatives.contains_key(&candidate) {
                    add_to_result(pos_num1, pos_num2, candidate);
                }
            }
        }

        // positive, negative and zero
        if zeros > 0 {
            for &neg_num in negatives.keys() {
                if positives.contains_key(&-neg_num) {
                    add_to_result(neg_num, 0, -neg_num);
                }
            }
        }

        result.into_iter().collect()
    }

    pub fn three_sum_3_optimized(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut negatives: HashMap<i32, i32> = HashMap::new();
        let mut positives: HashMap<i32, i32> = HashMap::new();
        let mut zeros = 0;

        for num in nums.into_iter() {
            if num < 0 {
                *negatives.entry(num).or_default() += 1;
            } else if num > 0 {
                *positives.entry(num).or_default() += 1;
            } else {
                zeros += 1;
            }
        }

        let mut neg_list = vec![];
        for (&neg, &freq) in negatives.iter() {
            let freq = freq.min(2);
            for _ in 0..freq {
                neg_list.push(neg);
            }
        }
        let mut pos_list = vec![];
        for (&pos, &freq) in positives.iter() {
            let freq = freq.min(2);
            for _ in 0..freq {
                pos_list.push(pos);
            }
        }

        let mut result = HashSet::new();

        let mut add_to_result = |v1, v2, v3| {
            let mut vals = vec![v1, v2, v3];
            vals.sort();
            result.insert(vals)
        };

        if zeros > 2 {
            add_to_result(0, 0, 0);
        }

        // two negatives and one positive
        for (idx1, &neg_num1) in neg_list.iter().enumerate() {
            for &neg_num2 in neg_list.iter().skip(idx1 + 1) {
                let candidate = -(neg_num1 + neg_num2);
                if positives.contains_key(&candidate) {
                    add_to_result(neg_num1, neg_num2, candidate);
                }
            }
        }

        // two positives and one negative
        for (idx1, &pos_num1) in pos_list.iter().enumerate() {
            for &pos_num2 in pos_list.iter().skip(idx1 + 1) {
                let candidate = -(pos_num1 + pos_num2);
                if negatives.contains_key(&candidate) {
                    add_to_result(pos_num1, pos_num2, candidate);
                }
            }
        }

        // positive, negative and zero
        if zeros > 0 {
            for &neg_num in negatives.keys() {
                if positives.contains_key(&-neg_num) {
                    add_to_result(neg_num, 0, -neg_num);
                }
            }
        }

        result.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::vec2d;

    use super::*;

    #[test]
    fn case1() {
        let nums = vec![-1, 0, 1, 2, -1, -4];
        let mut expected = vec2d![[-1, -1, 2], [-1, 0, 1]];
        expected.sort();

        let mut result = Solution::three_sum(nums);
        result.sort();

        assert_eq!(result, expected);
    }

    #[test]
    fn case2() {
        let nums = vec![3, 0, -2, -1, 1, 2];
        let mut expected = vec2d![[-2, -1, 3], [-2, 0, 2], [-1, 0, 1]];

        expected.sort();

        let mut result = Solution::three_sum(nums);
        result.sort();

        assert_eq!(result, expected);
    }
}

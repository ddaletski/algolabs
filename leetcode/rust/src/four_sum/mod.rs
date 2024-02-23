use std::collections::{HashMap, HashSet};

struct Solution;

impl Solution {
    pub fn is_unique(arr: &Vec<u8>) -> bool {
        for (idx, &item) in arr.iter().enumerate().skip(1) {
            if arr[idx - 1] == item {
                return false;
            }
        }

        true
    }

    pub fn remove_duplicates(arr: Vec<i32>) -> Vec<i32> {
        let mut freqs: HashMap<i32, i32> = HashMap::new();

        for num in arr {
            *freqs.entry(num).or_default() += 1;
        }

        let mut result = vec![];
        for (num, freq) in freqs {
            for _ in 0..(freq.min(4)) {
                result.push(num);
            }
        }

        result
    }

    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let nums = Solution::remove_duplicates(nums);

        let mut pairs_sums: HashMap<i32, Vec<(u8, u8)>> = HashMap::new();

        for (idx1, &num1) in nums.iter().enumerate() {
            for (idx2, &num2) in nums.iter().enumerate().skip(idx1 + 1) {
                pairs_sums
                    .entry(num1 + num2)
                    .or_default()
                    .push((idx1 as u8, idx2 as u8));
            }
        }

        let mut result: HashSet<Vec<i32>> = HashSet::new();

        for (&sum1, candidates1) in pairs_sums.iter() {
            let (sum2, err) = target.overflowing_sub(sum1);
            if err {
                continue;
            }

            if let Some(candidates2) = pairs_sums.get(&sum2) {
                for pair1 in candidates1 {
                    for pair2 in candidates2 {
                        let mut four = vec![pair1.0, pair1.1, pair2.0, pair2.1];
                        four.sort();
                        if !Solution::is_unique(&four) {
                            continue;
                        }
                        let mut four: Vec<i32> =
                            four.into_iter().map(|idx| nums[idx as usize]).collect();
                        four.sort();
                        result.insert(four);
                    }
                }
            }
        }

        result.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec2d;

    fn case(nums: &Vec<i32>, target: i32, expected: &Vec<Vec<i32>>) {
        let mut expected: Vec<Vec<i32>> = expected
            .clone()
            .into_iter()
            .map(|v| {
                let mut v_sorted = v;
                v_sorted.sort();
                v_sorted
            })
            .collect();
        expected.sort();

        let mut result: Vec<Vec<i32>> = Solution::four_sum(nums.clone(), target)
            .into_iter()
            .map(|v| {
                let mut v_sorted = v;
                v_sorted.sort();
                v_sorted
            })
            .collect();
        result.sort();

        assert_eq!(result, expected);
    }

    #[test]
    fn case1() {
        let nums = vec![1, 0, -1, 0, -2, 2];
        let target = 0;
        let expected_result = vec2d![[-2, -1, 1, 2], [-2, 0, 0, 2], [-1, 0, 0, 1]];
        case(&nums, target, &expected_result);
    }

    #[test]
    fn case2() {
        let nums = vec![2, 2, 2, 2, 2];
        let target = 8;
        let expected_result = vec2d![[2, 2, 2, 2]];
        case(&nums, target, &expected_result);
    }

    #[test]
    fn case3() {
        let nums = vec![-5, 5, 4, -3, 0, 0, 4, -2];
        let target = 4;
        let expected_result = vec2d![[-5, 0, 4, 5], [-3, -2, 4, 5]];
        case(&nums, target, &expected_result);
    }

    #[test]
    fn case4() {
        let nums = vec![
            2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
            2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
            2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
            2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
            2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
            2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
            2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
        ];
        let target = 8;
        let expected_result = vec2d![[2, 2, 2, 2]];
        case(&nums, target, &expected_result);
    }

    #[test]
    fn case5() {
        let nums = vec![1000000000, 1000000000, 1000000000, 1000000000];
        let target = -294967296;
        let expected_result = vec2d![];
        case(&nums, target, &expected_result);
    }
}

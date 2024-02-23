use std::collections::HashSet;

use crate::common::disjoint_sets_union::{HashMapDSU, UnionFind};

pub struct Solution {}

///////////////////////////////////

fn factors(num: usize) -> Vec<usize> {
    let mut result = vec![];

    let max_num = (f32::sqrt(num as f32 * 1.1)) as usize;
    for i in 2..=max_num {
        if num % i == 0 {
            result.push(i);
            result.push(num / i);
        }
    }

    result
}

////////////////////////////////////////

impl Solution {
    pub fn largest_component_size(nums: Vec<i32>) -> i32 {
        let nums_set: HashSet<usize> = nums.iter().map(|&n| n as usize).collect();

        let mut set = HashMapDSU::new();

        for num in nums.into_iter().map(|i| i as usize) {
            set.insert(num);
            for factor in factors(num) {
                set.join(num, factor);
            }
        }

        set.components()
            .values()
            .map(|component| {
                // filter out factors leaving only numbers from initial list
                component
                    .into_iter()
                    .filter_map(|number| nums_set.get(number))
                    .count()
            })
            .max()
            .unwrap_or(0) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(vec![4,6,15,35], 4)]
    #[case(vec![20,50,9,63], 2)]
    #[case(vec![2,3,6,7,4,12,21,39], 8)]
    #[case(vec![1,2,3,4,5,6,7,8,9], 6)]
    #[case(vec![1,2,3,5,7,11,13,17,19], 1)]
    fn cases(#[case] nums: Vec<i32>, #[case] expected_result: i32) {
        let result = Solution::largest_component_size(nums);
        assert_eq!(result, expected_result);
    }
}

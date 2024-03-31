use std::{
    collections::{BinaryHeap, HashMap},
    iter::zip,
};

struct Solution;

impl Solution {
    pub fn most_frequent_i_ds(nums: Vec<i32>, freq: Vec<i32>) -> Vec<i64> {
        let mut result = vec![];

        let mut freqs = HashMap::with_capacity(nums.len());
        let mut heap = BinaryHeap::with_capacity(nums.len());

        for (id, freq_change) in zip(nums, freq) {
            let freq_change = freq_change as i64;
            let freq = *freqs
                .entry(id)
                .and_modify(|x| *x += freq_change)
                .or_insert(freq_change);

            heap.push((freq, id));

            while let Some(&(freq, id)) = heap.peek() {
                if freq != *freqs.get(&id).unwrap() {
                    heap.pop();
                } else {
                    break;
                }
            }
            if let Some(&(freq, _)) = heap.peek() {
                result.push(freq);
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use algo_toolbox::assert_returns;

    use super::*;

    #[test]
    fn case1() {
        assert_returns!(
            vec![3, 3, 2, 2],
            Solution::most_frequent_i_ds,
            vec![2, 3, 2, 1],
            vec![3, 2, -3, 1]
        );
    }

    #[test]
    fn case2() {
        assert_returns!(
            vec![2, 0, 1],
            Solution::most_frequent_i_ds,
            vec![5, 5, 3],
            vec![2, -2, 1]
        );
    }
}

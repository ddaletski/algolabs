pub struct Solution {}

/////////////////////////////////////////////////////
use std::collections::HashMap;

impl Solution {
    pub fn min_set_size(arr: Vec<i32>) -> i32 {
        let arr_size = arr.len();

        let freqs = arr.into_iter().fold(HashMap::new(), |mut map, number| {
            *map.entry(number).or_insert(0) += 1;
            map
        });

        let mut freqs_vec: Vec<_> = freqs.into_iter().map(|(_, freq)| freq).collect();
        freqs_vec.sort_unstable();


        let mut removed = 0;
        let mut items_left = (arr_size / 2) as i32;
        while items_left > 0 {
            items_left -= freqs_vec.pop().unwrap();
            removed += 1
        }

        removed
    }
}

/////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(vec![3,3,3,3,5,5,5,2,2,7], 2)]
    #[case(vec![7,7,7,7,7,7], 1)]
    fn cases(#[case] arr: Vec<i32>, #[case] expected_result: i32) {
        let result = Solution::min_set_size(arr);
        assert_eq!(result, expected_result);
    }
}

struct Solution;

impl Solution {
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        let mut min = i32::MAX;
        let mut middle = i32::MAX;

        for num in nums {
            if num <= min {
                min = num;
            } else if num <= middle {
                middle = num;
            } else {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use algo_toolbox::assert_returns;

    #[test]
    fn case1() {
        assert_returns!(true, Solution::increasing_triplet, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn case2() {
        assert_returns!(false, Solution::increasing_triplet, vec![5, 4, 3, 2, 1]);
    }

    #[test]
    fn case3() {
        assert_returns!(true, Solution::increasing_triplet, vec![2, 1, 5, 0, 4, 6]);
    }
}

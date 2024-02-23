pub struct Solution {}

impl Solution {
    pub fn can_reach(mut arr: Vec<i32>, start: i32) -> bool {
        Solution::can_reach_impl(&mut arr, start)
    }

    fn can_reach_impl(arr: &mut Vec<i32>, start: i32) -> bool {
        let idx = start as usize;

        if arr[idx] == -1 {
            return false;
        } else if arr[idx] == 0 {
            return true;
        }

        let step = arr[idx];
        arr[idx] = -1;

        let left = start - step;
        let right = start + step;

        (left >= 0 && Solution::can_reach_impl(arr, left))
            || (right < (arr.len() as i32) && Solution::can_reach_impl(arr, right))
    }
}

#[cfg(test)]
mod tests {
    use crate::assert_returns;

    use super::*;

    #[test]
    fn case1() {
        let arr = vec![4, 2, 3, 0, 3, 1, 2];
        let start = 5;

        assert_returns!(true, Solution::can_reach, arr.clone(), start);
    }

    #[test]
    fn case2() {
        let arr = vec![4, 2, 3, 0, 3, 1, 2];
        let start = 0;

        assert_returns!(true, Solution::can_reach, arr.clone(), start);
    }

    #[test]
    fn case3() {
        let arr = vec![3, 0, 2, 1, 2];
        let start = 2;

        assert_returns!(false, Solution::can_reach, arr.clone(), start);
    }
}

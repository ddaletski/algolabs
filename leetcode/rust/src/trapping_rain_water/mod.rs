struct Solution;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        if height.len() <= 2 {
            return 0;
        }

        let mut max_left = 0;
        let mut max_right = 0;

        let mut left = 0;
        let mut right = height.len() - 1;

        let mut total = 0;

        while left <= right {
            let left_height = height[left];
            let right_height = height[right];

            total += if max_left < max_right {
                let vol = (max_left.min(max_right) - left_height).max(0);
                left += 1;
                max_left = max_left.max(left_height);

                vol
            } else {
                let vol = (max_left.min(max_right) - right_height).max(0);
                right -= 1;
                max_right = max_right.max(right_height);

                vol
            }
        }

        total
    }
}

#[cfg(test)]
mod tests {
    use crate::common;

    use super::*;

    #[test]
    fn case1() {
        assert_eq!(Solution::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
    }

    #[test]
    fn case2() {
        assert_eq!(Solution::trap(vec![4, 2, 0, 3, 2, 5]), 9);
    }

    #[test]
    fn case3() {
        assert_eq!(Solution::trap(vec![5, 4, 1, 2]), 1);
    }

    #[test]
    fn case4() {
        assert_eq!(Solution::trap(vec![0]), 0);
    }

    #[test]
    fn case5() {
        assert_eq!(Solution::trap(vec![1, 2]), 0);
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        let rand_list = common::random_uniform_list(10000, 0, 100);

        b.iter(|| Solution::trap(rand_list.clone()));
    }
}

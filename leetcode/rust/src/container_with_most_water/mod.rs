struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = height.len() - 1;

        let mut max_area = 0;

        while left < right {
            let left_height = height[left];
            let right_height = height[right];
            let area = (right - left) as i32 * left_height.min(right_height);

            if area > max_area {
                max_area = area;
            }

            if left_height < right_height {
                left += 1;
            } else {
                right -= 1;
            }
        }

        max_area
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;
    use rand::{Rng, SeedableRng};

    use super::Solution;

    #[test]
    fn case1() {
        assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
    }

    #[test]
    fn case2() {
        assert_eq!(Solution::max_area(vec![1, 1]), 1);
    }

    #[test]
    fn case3() {
        assert_eq!(Solution::max_area(vec![4, 3, 2, 1, 4]), 16);
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        let rand_list = rand::rngs::StdRng::seed_from_u64(0)
            .sample_iter(rand::distributions::Uniform::new(0, 1000))
            .take(10000)
            .collect_vec();

        b.iter(|| Solution::max_area(rand_list.clone()));
    }
}

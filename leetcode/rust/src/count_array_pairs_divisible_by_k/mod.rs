use std::collections::HashMap;

struct Solution;

fn gcd(a: i32, b: i32) -> i32 {
    let (a, b) = if a < b { (b, a) } else { (a, b) };

    fn gcd_impl(a: i32, b: i32) -> i32 {
        if b == 0 {
            a
        } else {
            gcd_impl(b, a % b)
        }
    }

    gcd_impl(a, b)
}

impl Solution {
    pub fn count_pairs(nums: Vec<i32>, k: i32) -> i64 {
        let mut count = 0;

        let mut gcd_counter = HashMap::new();
        for &n in &nums {
            let gcd = gcd(n, k);
            let rest = k / gcd;

            for (&factor, &factor_freq) in &gcd_counter {
                if factor % rest == 0 {
                    count += factor_freq;
                }
            }

            gcd_counter.entry(gcd).and_modify(|x| *x += 1).or_insert(1);
        }

        count
    }

    pub fn count_pairs_2(nums: Vec<i32>, k: i32) -> i64 {
        let mut count = 0;

        let mut gcd_counter = HashMap::new();
        for &n in &nums {
            let gcd = gcd(n, k) as i64;
            gcd_counter.entry(gcd).and_modify(|x| *x += 1).or_insert(1);
        }

        let k = k as i64;

        for (idx1, (&gcd1, &count1)) in gcd_counter.iter().enumerate() {
            for (&gcd2, &count2) in gcd_counter.iter().skip(idx1 + 1) {
                if (gcd1 * gcd2) % k == 0 {
                    count += count1 * count2;
                }
            }

            if count1 > 1 && (gcd1 * gcd1) % k == 0 {
                count += count1 * (count1 - 1) / 2;
            }
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use algo_toolbox::assert_returns;

    #[test]
    fn case1_1() {
        assert_returns!(7, Solution::count_pairs, vec![1, 2, 3, 4, 5], 2);
    }

    #[test]
    fn case1_2() {
        assert_returns!(0, Solution::count_pairs, vec![1, 2, 3, 4], 5);
    }

    #[test]
    fn case2_1() {
        assert_returns!(7, Solution::count_pairs_2, vec![1, 2, 3, 4, 5], 2);
    }

    #[test]
    fn case2_2() {
        assert_returns!(0, Solution::count_pairs_2, vec![1, 2, 3, 4], 5);
    }
}

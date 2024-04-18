struct Solution;

impl Solution {
    pub fn get_sum(a: i32, b: i32) -> i32 {
        let a = (a + 1000) as u32;
        let b = (b + 1000) as u32;

        fn rec(a: u32, b: u32, c: u32) -> u32 {
            if a | b | c == 0 {
                return 0;
            }

            let bit1 = a % 2;
            let bit2 = b % 2;

            let or = bit1 | bit2;
            let xor = bit1 ^ bit2;
            let and = bit1 & bit2;

            if c == 0 {
                (rec(a >> 1, b >> 1, and) << 1) | (xor % 2)
            } else {
                (rec(a >> 1, b >> 1, or) << 1) | (!xor % 2)
            }
        }

        rec(a, b, 0) as i32 - 2000
    }
}

#[cfg(test)]
mod tests {
    proptest::proptest! {
        #[test]
        fn test(a in -1000..=1000, b in -1000..=1000) {
            assert_eq!(super::Solution::get_sum(a, b), a + b);
        }
    }
}

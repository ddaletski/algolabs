pub struct Solution {}

/////////////////////////////////////////////////////
impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        if n < 3 {
            return 0;
        }

        let n = n as usize;

        let mut sieve: Vec<bool> = Vec::with_capacity(n);
        sieve.resize(n, true);
        sieve[0] = false;
        sieve[1] = false;

        for i in 2..n {
            if !sieve[i] {
                continue;
            }

            for multiple in sieve[i..n].iter_mut().step_by(i).skip(1) {
                *multiple = false;
            }
        }

        sieve.into_iter().fold(0, |init, item| init + item as i32)
    }
}
//////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_returns;
    use rstest::rstest;

    #[rstest]
    #[case(10, 4)]
    #[case(0, 0)]
    #[case(1, 0)]
    #[case(2, 0)]
    #[case(3, 1)]
    #[case(4, 2)]
    #[case(6, 3)]
    fn it_works(#[case] n: i32, #[case] expected_result: i32) {
        assert_returns!(expected_result, Solution::count_primes, n);
    }
}

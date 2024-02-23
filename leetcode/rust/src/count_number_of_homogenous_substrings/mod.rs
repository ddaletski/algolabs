pub struct Solution {}

//////////////////////////////////

const MODULO: usize = 1000000000 + 7;

impl Solution {
    pub fn max_homogenous_strings_for_len(len: usize) -> usize {
        len.wrapping_mul(len + 1) / 2
    }

    pub fn count_homogenous(s: String) -> i32 {
        let mut total_count = 0;

        let mut current_char = (None, 0);
        for ch in s.chars().peekable() {
            match current_char.0 {
                Some(curr_ch) => {
                    if curr_ch == ch {
                        current_char.1 += 1;
                    } else {
                        let last_count = current_char.1;
                        total_count = (total_count
                            + Solution::max_homogenous_strings_for_len(last_count))
                            % MODULO;
                        current_char = (Some(ch), 1);
                    }
                }
                None => {
                    current_char = (Some(ch), 1);
                }
            }
        }
        let last_count = current_char.1;
        total_count = (total_count + Solution::max_homogenous_strings_for_len(last_count)) % MODULO;

        total_count as i32
    }
}

/////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("abbcccaa", 13)]
    #[case("xy", 2)]
    #[case("zzzzz", 15)]
    #[case("a", 1)]
    #[case("aa", 3)]
    #[case("aaa", 6)]
    fn cases(#[case] string: String, #[case] expected_result: i32) {
        let result = Solution::count_homogenous(string);
        assert_eq!(result, expected_result);
    }

    #[rstest]
    #[case(1, 1)]
    #[case(2, 3)]
    #[case(3, 6)]
    #[case(4, 10)]
    #[case(5, 15)]
    #[case(6, 21)]
    #[case(7, 28)]
    #[case(100000, 49965)]
    fn test_max_homogenous_strings_for_len(#[case] len: i32, #[case] expected_result: i32) {
        let result = Solution::max_homogenous_strings_for_len(len as usize) % MODULO;
        assert_eq!(result as i32, expected_result);
    }
}

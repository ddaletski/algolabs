struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let bytes = s.as_bytes();

        let mut left = 0;
        let mut right = left;

        let mut chars_met = vec![false; 256];
        let mut longest = 0;

        while right < bytes.len() {
            let current_char_index = bytes[right] as usize;
            while chars_met[current_char_index] {
                chars_met[bytes[left] as usize] = false;
                left += 1;
            }
            chars_met[current_char_index] = true;
            right += 1;
            longest = longest.max(right - left);
        }

        longest as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::common::random_uniform_list;

    use super::*;
    use algo_toolbox::assert_returns;
    use rstest::rstest;

    #[rstest]
    #[case("abcabcbb", 3)]
    #[case("bbbbb", 1)]
    #[case("pwwkew", 3)]
    fn it_works(#[case] s: String, #[case] expected: i32) {
        assert_returns!(expected, Solution::length_of_longest_substring, s);
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        let random_string: String = random_uniform_list::<char>(10000, 0 as char, 255 as char)
            .into_iter()
            .collect();

        b.iter(|| Solution::length_of_longest_substring(random_string.clone()));
    }
}

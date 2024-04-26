struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut longest = 0;
        let mut positions = [-1i32; 256];

        let mut start = 0i32;
        for (pos, ch) in s.into_bytes().into_iter().enumerate() {
            let pos = pos as i32;
            let other_pos = positions[ch as usize];
            if other_pos < start {
                // either not met yet or outdated
                longest = longest.max(pos - start + 1);
            } else {
                start = other_pos + 1;
            }
            positions[ch as usize] = pos;
        }

        longest as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{assert_returns, common::random_uniform_list};

    #[rstest::rstest]
    #[case("abcabcbb", 3)]
    #[case("bbbbb", 1)]
    #[case("pwwkew", 3)]
    #[case(" ", 1)]
    #[case("au", 2)]
    fn case(#[case] s: &str, #[case] expected: i32) {
        assert_returns!(
            expected,
            Solution::length_of_longest_substring,
            s.to_string()
        );
    }

    #[bench]
    fn bench(b: &mut ::test::Bencher) {
        let random_string: String = random_uniform_list::<char>(10000, 0 as char, 255 as char)
            .into_iter()
            .collect();

        b.iter(|| Solution::length_of_longest_substring(random_string.clone()));
    }
}

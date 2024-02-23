pub struct Solution {}

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.is_empty() {
            return "".to_owned();
        }

        let mut prefix: Vec<u8> = strs
            .iter()
            .min_by_key(|&s| s.len())
            .unwrap()
            .as_bytes()
            .to_owned();

        for s in strs.into_iter().map(|s| s.into_bytes()) {
            for idx in (0..prefix.len()).rev() {
                if s[idx] != prefix[idx] {
                    prefix.drain(idx..);
                }
            }
            if prefix.is_empty() {
                break;
            }
        }

        String::from_utf8(prefix).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_returns;

    #[test]
    fn case1() {
        let input = vec!["flower", "flow", "flight"]
            .into_iter()
            .map(|s| s.to_owned())
            .collect();
        let expected = "fl".to_owned();
        assert_returns!(expected, Solution::longest_common_prefix, input);
    }

    #[test]
    fn case2() {
        let input = vec!["dog","racecar","car"]
            .into_iter()
            .map(|s| s.to_owned())
            .collect();
        let expected = "".to_owned();
        assert_returns!(expected, Solution::longest_common_prefix, input);
    }

    #[test]
    fn case3() {
        let input = vec!["cir", "car"]
            .into_iter()
            .map(|s| s.to_owned())
            .collect();
        let expected = "c".to_owned();
        assert_returns!(expected, Solution::longest_common_prefix, input);
    }
}

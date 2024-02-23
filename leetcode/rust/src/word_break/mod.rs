pub struct Solution;

////////////////////////////////////////////////

use std::collections::HashSet;

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let mut dict: HashSet<String> = word_dict.iter().map(|s| s.to_owned()).collect();
        for word in &word_dict {
            dict = Solution::_filter_dict(word, dict);
        }

        let dict: Vec<String> = dict.into_iter().collect();
        Solution::_impl(&s, &dict)
    }

    fn _impl(s: &str, dict: &[String]) -> bool {
        if s.is_empty() {
            return true;
        }

        for word in dict {
            if s.starts_with(word) {
                let word_len = word.len();
                if Solution::_impl(&s[word_len..], dict) {
                    return true;
                }
            }
        }

        false
    }

    fn _filter_dict(anchor: &str, mut dict: HashSet<String>) -> HashSet<String> {
        let mut to_remove: HashSet<String> = HashSet::new();

        for word in &dict {
            if anchor == word {
                continue;
            }

            if Solution::_consists_of(&anchor, &word) {
                to_remove.insert(word.to_owned());
            }
        }

        for word in to_remove {
            dict.remove(&word);
        }

        dict
    }

    fn _consists_of(part: &str, word: &str) -> bool {
        if word.len() % part.len() != 0 {
            false
        } else {
            Solution::_consists_of_rec(part, word)
        }
    }

    fn _consists_of_rec(part: &str, word: &str) -> bool {
        if word.is_empty() {
            true
        } else if word.starts_with(part) {
            Solution::_consists_of_rec(part, &word[part.len()..])
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_returns;
    use rstest::rstest;

    #[rstest]
    #[case("leetcode", vec!["leet","code"], true)]
    #[case("applepenapple", vec!["apple","pen"], true)]
    #[case("catsandog", vec!["cats","dog","sand","and","cat"], false)]
    #[case("", vec!["cats"], true)]
    #[case("", vec![], true)]
    #[case("a", vec!["a"], true)]
    #[case("a", vec![], false)]
    #[case("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaab", vec!["a","aa","aaa","aaaa","aaaaa","aaaaaa","aaaaaaa","aaaaaaaa","aaaaaaaaa","aaaaaaaaaa"], false)]
    fn it_works(#[case] text: String, #[case] dict: Vec<&str>, #[case] expected_answer: bool) {
        let dict: Vec<String> = dict.into_iter().map(|s| s.to_owned()).collect();

        assert_returns!(expected_answer, Solution::word_break, text, dict);
    }
}

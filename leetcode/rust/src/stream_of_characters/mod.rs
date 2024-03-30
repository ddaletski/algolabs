use std::collections::LinkedList;

use algo_toolbox::trie::{
    hash_trie::{HashTrie, HashTrieNode},
    StringSet,
};

/////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct StreamChecker {
    trie: HashTrie<char>,
    current_matches: LinkedList<*const HashTrieNode<char>>,
}

impl StreamChecker {
    pub fn new(words: Vec<String>) -> Self {
        let mut trie = HashTrie::new();
        for word in words {
            trie.insert(&word);
        }

        StreamChecker {
            trie,
            current_matches: LinkedList::new(),
        }
    }

    pub fn query(&mut self, letter: char) -> bool {
        let mut result = false;
        unsafe {
            for node in std::mem::replace(&mut self.current_matches, LinkedList::new()) {
                if let Some(child) = node.as_ref().unwrap().find_prefix(std::iter::once(letter)) {
                    self.current_matches.push_back(child);

                    if child.word_end() {
                        result = true;
                    }
                }
            }

            if let Some(word_start) = self.trie.root().find_prefix(std::iter::once(letter)) {
                self.current_matches.push_back(word_start);
                if word_start.word_end() {
                    result = true;
                }
            }
        }

        result
    }
}

//////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::StreamChecker;
    use crate::assert_returns;
    use rstest::fixture;
    use rstest::rstest;

    #[fixture]
    fn words() -> Vec<String> {
        vec!["cd", "f", "kl"]
            .iter()
            .map(|&s| s.to_owned())
            .collect()
    }

    #[fixture]
    fn cases() -> Vec<(char, bool)> {
        let chars = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l'];
        let results = vec![
            false, false, false, true, false, true, false, false, false, false, false, true,
        ];
        assert!(chars.len() == results.len());

        chars.into_iter().zip(results).collect()
    }

    #[rstest]
    fn it_works(words: Vec<String>, cases: Vec<(char, bool)>) {
        let mut checker = StreamChecker::new(words);

        for (sym, expected_return_value) in cases {
            assert_returns!(
                expected_return_value,
                StreamChecker::query,
                &mut checker,
                sym
            );
        }
    }
}

use crate::common::trie::{CharTrie, TrieNode};

#[derive(Debug)]
struct MagicDictionary {
    trie: CharTrie,
}

struct SearchState<'a> {
    node: &'a TrieNode<char>,
    word_pos: usize,
    replacement_left: bool,
}

impl MagicDictionary {
    pub fn new() -> Self {
        MagicDictionary { trie: CharTrie::new() }
    }

    pub fn build_dict(&mut self, dictionary: Vec<String>) {
        for s in dictionary {
            self.trie.insert(&s);
        }
    }

    pub fn search(&self, search_word: String) -> bool {
        let chars: Vec<char> = search_word.chars().collect();

        let mut stack: Vec<SearchState> = vec![];

        stack.push(SearchState {
            node: self.trie.root(),
            word_pos: 0,
            replacement_left: true,
        });

        while let Some(current_state) = stack.pop() {
            if current_state.word_pos == chars.len() {
                if current_state.node.word_end && !current_state.replacement_left {
                    return true;
                } else {
                    continue;
                }
            }

            let current_char = chars[current_state.word_pos];

            if current_state.replacement_left {
                for next_char in ('a'..='z').filter(|&chr| chr != current_char) {
                    if let Some(next_node) = current_state.node.next(next_char) {
                        stack.push(SearchState {
                            node: next_node,
                            word_pos: current_state.word_pos + 1,
                            replacement_left: false,
                        });
                    }
                }
            }

            if let Some(next_node) = current_state.node.next(current_char) {
                stack.push(SearchState {
                    node: next_node,
                    word_pos: current_state.word_pos + 1,
                    replacement_left: current_state.replacement_left,
                });
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_returns;
    use rstest::{fixture, rstest};

    #[fixture]
    fn words() -> Vec<String> {
        vec!["hello", "leetcode"]
            .into_iter()
            .map(|s| s.to_owned())
            .collect()
    }

    #[fixture]
    fn full_dict(words: Vec<String>) -> MagicDictionary {
        let mut dict = MagicDictionary::new();
        dict.build_dict(words);
        dict
    }

    #[rstest]
    #[case("hellou", false)]
    #[case("hello", false)]
    #[case("hhllo", true)]
    #[case("hell", false)]
    #[case("leetcoded", false)]
    #[case("leecode", false)]
    fn case(full_dict: MagicDictionary, #[case] query: String, #[case] expected_result: bool) {
        assert_returns!(expected_result, MagicDictionary::search, &full_dict, query);
    }
}

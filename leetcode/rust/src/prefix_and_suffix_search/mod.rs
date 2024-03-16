use std::collections::{HashMap, HashSet};

use algo_toolbox::trie::CharTrie;

#[allow(unused)]
struct WordFilter1 {
    forward_trie: CharTrie,
    backward_trie: CharTrie,
    index_mapping: HashMap<String, i32>,
}

#[allow(unused)]
impl WordFilter1 {
    fn new(words: Vec<String>) -> Self {
        let mut fw = CharTrie::new();
        let mut bw = CharTrie::new();

        words.iter().for_each(|word| {
            let word_reversed: String = word.chars().rev().collect();
            fw.insert(&word);
            bw.insert(&word_reversed);
        });

        let map: HashMap<String, i32> = words
            .into_iter()
            .enumerate()
            .map(|(idx, word)| (word, idx as i32))
            .collect();

        Self {
            forward_trie: fw,
            backward_trie: bw,
            index_mapping: map,
        }
    }

    fn f(&self, pref: String, suff: String) -> i32 {
        let suffix_reversed: String = suff.chars().rev().collect();

        let prefix_set: HashSet<String> = self.forward_trie.find_all(&pref).into_iter().collect();
        let suffix_set: HashSet<String> = self
            .backward_trie
            .find_all(&suffix_reversed)
            .into_iter()
            .map(|word_rev| word_rev.chars().rev().collect())
            .collect();

        let common_set = prefix_set.intersection(&suffix_set);

        let max_index = common_set
            .map(|word| *self.index_mapping.get(word).unwrap_or(&-1))
            .max()
            .unwrap_or(-1);

        max_index
    }
}

#[allow(unused)]
struct WordFilter2 {
    prefixes: HashMap<String, HashSet<u16>>,
    suffixes: HashMap<String, HashSet<u16>>,
}

#[allow(unused)]
impl WordFilter2 {
    fn new(words: Vec<String>) -> Self {
        let words: HashMap<String, i32> = words
            .into_iter()
            .enumerate()
            .map(|(idx, word)| (word, idx as i32))
            .collect();

        let mut prefixes: HashMap<String, HashSet<u16>> = HashMap::new();

        for (word, &word_idx) in words.iter() {
            for pref_len in 1..=7 {
                if word.len() < pref_len {
                    continue;
                }

                let prefix: String = word.chars().take(pref_len).collect();

                prefixes.entry(prefix).or_default().insert(word_idx as u16);
            }
        }

        let mut suffixes: HashMap<String, HashSet<u16>> = HashMap::new();

        for (word, &word_idx) in words.iter() {
            for suff_len in 1..=7 {
                if word.len() < suff_len {
                    continue;
                }

                let suffix: String = word
                    .bytes()
                    .skip(word.len() - suff_len)
                    .map(|byte| byte as char)
                    .collect();

                suffixes.entry(suffix).or_default().insert(word_idx as u16);
            }
        }

        Self { prefixes, suffixes }
    }

    fn f(&self, pref: String, suff: String) -> i32 {
        let prefix_set = self.prefixes.get(&pref).cloned().unwrap_or_default();
        let suffix_set = self.suffixes.get(&suff).cloned().unwrap_or_default();

        let common_set = prefix_set.intersection(&suffix_set);

        common_set.max().cloned().map(i32::from).unwrap_or(-1)
    }
}

#[allow(unused)]
type WordFilter = WordFilter2;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let words = vec!["lol", "apple", "kek", "ape", "chebureck"]
            .into_iter()
            .map(String::from)
            .collect();
        let filter = WordFilter::new(words);

        assert_eq!(filter.f("a".into(), "e".into()), 3);
    }
}

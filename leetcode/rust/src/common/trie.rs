use std::{collections::HashMap, fmt::Debug, hash::Hash};

pub struct TrieNode<Char> {
    pub character: Char,
    pub word_end: bool,
    children: HashMap<Char, Box<TrieNode<Char>>>,
}

impl<Char> TrieNode<Char>
where
    Char: Eq + Hash + Clone,
{
    pub fn new(character: Char, word_end: bool) -> Self {
        TrieNode {
            character,
            word_end,
            children: HashMap::new(),
        }
    }

    pub fn insert(&mut self, word: impl Iterator<Item = Char>) -> bool {
        self.insert_impl(word)
    }

    pub fn contains(&self, word: impl Iterator<Item = Char>) -> bool {
        if let Some(node) = self.find_impl(word) {
            node.word_end
        } else {
            false
        }
    }

    pub fn find_prefix(&self, prefix: impl Iterator<Item = Char>) -> Option<&Self> {
        self.find_impl(prefix)
    }

    pub fn next(&self, next_char: Char) -> Option<&Self> {
        self.children.get(&next_char).map(|b| b.as_ref())
    }

    pub fn children_count(&self) -> usize {
        self.children.len()
    }

    pub fn children(&self) -> impl Iterator<Item = (&Char, &TrieNode<Char>)> {
        self.children.iter().map(|(k, v)| (k, v.as_ref()))
    }

    fn insert_impl(&mut self, mut word: impl Iterator<Item = Char>) -> bool {
        if let Some(next_char) = word.next() {
            match &mut self.children.get_mut(&next_char) {
                Some(next_node) => {
                    return next_node.insert_impl(word);
                }
                None => {
                    let mut next_node = Box::new(TrieNode::new(next_char.clone(), false));
                    let result = next_node.insert_impl(word);

                    self.children.insert(next_char, next_node);
                    return result;
                }
            }
        }

        self.word_end = true;
        true
    }

    fn find_impl(&self, mut word: impl Iterator<Item = Char>) -> Option<&Self> {
        if let Some(next_char) = word.next() {
            if let Some(next_node) = &self.children.get(&next_char) {
                return next_node.find_impl(word);
            } else {
                return None;
            }
        }
        Some(self)
    }

    fn fill_all_children(&self, current_word: &mut Vec<Char>, result: &mut Vec<Vec<Char>>) {
        if self.word_end {
            result.push(current_word.clone());
        }

        self.children.iter().for_each(|(next_char, next_node)| {
            current_word.push(next_char.clone());
            next_node.fill_all_children(current_word, result);
            current_word.pop();
        });
    }
}

impl<Char> TrieNode<Char>
where
    Char: Debug,
{
    fn format_impl(&self, indent: usize, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let space = "| ".repeat(indent);
        f.write_fmt(format_args!("{}{:?}\n", space, self.character))?;

        for node in self.children.values() {
            node.format_impl(indent + 1, f)?;
        }

        Ok(())
    }
}

impl<Char: Debug> Debug for TrieNode<Char> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.format_impl(0, f)?;
        Ok(())
    }
}

pub struct Trie<Char> {
    root: TrieNode<Char>,
    words_count: usize,
}

impl<Char> Trie<Char>
where
    Char: Eq + Hash + Clone + Debug + Default,
{
    pub fn new() -> Self {
        Trie {
            root: TrieNode::new(Char::default(), false),
            words_count: 0,
        }
    }

    pub fn insert(&mut self, word: impl Iterator<Item = Char>) -> bool {
        let inserted = self.root.insert(word);

        if inserted {
            self.words_count += 1;
        }

        inserted
    }

    pub fn contains(&self, word: impl Iterator<Item = Char>) -> bool {
        self.root.contains(word)
    }

    pub fn root(&self) -> &TrieNode<Char> {
        &self.root
    }

    pub fn find_prefix(&self, prefix: impl Iterator<Item = Char>) -> Option<&TrieNode<Char>> {
        self.root.find_prefix(prefix)
    }

    pub fn find_all(&self, prefix: impl Iterator<Item = Char>) -> Vec<Vec<Char>> {
        let mut result = vec![];

        let prefix_owned = prefix.collect::<Vec<_>>();

        if let Some(starting_point) = self.root.find_prefix(prefix_owned.iter().cloned()) {
            let mut current_word = prefix_owned;
            starting_point.fill_all_children(&mut current_word, &mut result);
        }

        result
    }

    pub fn len(&self) -> usize {
        self.words_count
    }

    pub fn is_empty(&self) -> bool {
        self.words_count == 0
    }
}

impl<Char> Default for Trie<Char>
where
    Char: Eq + Hash + Clone + Debug + Default,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<Char: Debug> Debug for Trie<Char> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.root.fmt(f)?;
        Ok(())
    }
}

pub struct CharTrie(Trie<char>);

impl CharTrie {
    pub fn new() -> Self {
        Self(Trie::new())
    }

    pub fn insert(&mut self, word: &str) -> bool {
        self.0.insert(word.chars())
    }

    pub fn contains(&self, word: &str) -> bool {
        self.0.contains(word.chars())
    }

    pub fn root(&self) -> &TrieNode<char> {
        &self.0.root
    }

    pub fn find_prefix(&self, prefix: &str) -> Option<&TrieNode<char>> {
        self.0.find_prefix(prefix.chars())
    }

    pub fn find_all(&self, prefix: &str) -> Vec<String> {
        self.0
            .find_all(prefix.chars())
            .into_iter()
            .map(|chars| chars.into_iter().collect::<String>())
            .collect()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl Debug for CharTrie {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.root.fmt(f)?;
        Ok(())
    }
}

impl From<Vec<&str>> for CharTrie {
    fn from(value: Vec<&str>) -> Self {
        let mut trie = CharTrie::new();
        value.into_iter().for_each(|s| {
            trie.insert(s);
        });

        trie
    }
}

#[cfg(test)]
mod test {
    use crate::assert_returns;
    use rstest::{fixture, rstest};

    use lazy_static::lazy_static;
    use proptest::proptest;
    use rand::{
        self,
        distributions::Alphanumeric,
        distributions::{DistString, Uniform},
        prelude::Distribution,
    };

    use super::CharTrie;

    lazy_static! {
        static ref RAND_WORDS: Vec<String> = {
            let mut rng = rand::thread_rng();
            let len_distribution = Uniform::<usize>::from(1..10);

            let mut random_words: Vec<String> = vec![];

            for _ in 0..=100 {
                let word_len = len_distribution.sample(&mut rng);
                let word = Alphanumeric.sample_string(&mut rng, word_len);
                random_words.push(word);
            }

            random_words
        };
        static ref WORDS_100: Vec<String> = {
            let words_file_content = include_str!("testdata/words100.txt");

            words_file_content
                .trim()
                .split("\n")
                .map(|s| s.to_owned())
                .collect()
        };
    }

    #[fixture]
    fn top100words() -> Vec<&'static str> {
        WORDS_100.iter().map(|s| s.as_str()).collect()
    }

    #[rstest]
    fn top100words_has_100_words(top100words: Vec<&str>) {
        assert_eq!(top100words.len(), 100);
    }

    #[fixture]
    fn random_words() -> Vec<&'static str> {
        RAND_WORDS.iter().map(|s| s.as_str()).collect()
    }

    #[fixture]
    fn lol_kek_chebureck_list() -> Vec<&'static str> {
        vec!["lol", "kek", "chebureck"]
    }

    #[fixture]
    fn top100trie(top100words: Vec<&str>) -> CharTrie {
        CharTrie::from(top100words)
    }

    #[fixture]
    fn random_trie(random_words: Vec<&str>) -> CharTrie {
        CharTrie::from(random_words)
    }

    #[fixture]
    fn lol_kek_chebureck_trie(lol_kek_chebureck_list: Vec<&str>) -> CharTrie {
        CharTrie::from(lol_kek_chebureck_list)
    }

    mod trie_contains_inserted_words {
        use super::*;

        #[rstest]
        fn random(random_trie: CharTrie, random_words: Vec<&str>) {
            for word in random_words {
                assert_returns!(true, CharTrie::contains, &random_trie, word);
            }
        }

        #[rstest]
        fn top100(top100trie: CharTrie, top100words: Vec<&str>) {
            for word in top100words {
                assert_returns!(true, CharTrie::contains, &top100trie, word);
            }
        }
    }

    mod trie_finds_inserted_words {
        use super::*;

        #[rstest]
        fn random(random_trie: CharTrie, random_words: Vec<&str>) {
            for word in random_words {
                let returned_node = random_trie.find_prefix(word);
                assert!(returned_node.is_some());
                assert!(returned_node.unwrap().word_end);
            }
        }

        #[rstest]
        fn top100(top100trie: CharTrie, top100words: Vec<&str>) {
            for word in top100words {
                let returned_node = top100trie.find_prefix(word);
                assert!(returned_node.is_some());
                assert!(returned_node.unwrap().word_end);
            }
        }
    }

    #[rstest]
    fn find_incomplete_word_returns_node_with_word_end_equals_false(
        lol_kek_chebureck_trie: CharTrie,
        lol_kek_chebureck_list: Vec<&str>,
    ) {
        for word in lol_kek_chebureck_list {
            let incomplete_word = &word[0..word.len() - 1];
            let found_node = lol_kek_chebureck_trie.find_prefix(incomplete_word);

            assert!(found_node.is_some());
            assert!(!found_node.unwrap().word_end);
        }
    }

    mod find_all {
        use super::*;

        #[rstest]
        fn empty_prefix_populates_all_words(
            lol_kek_chebureck_trie: CharTrie,
            lol_kek_chebureck_list: Vec<&str>,
        ) {
            let mut expected_result: Vec<String> = lol_kek_chebureck_list
                .into_iter()
                .map(String::from)
                .collect();
            expected_result.sort();

            let mut result = lol_kek_chebureck_trie.find_all("");
            result.sort();

            assert_eq!(result, expected_result);
        }

        #[rstest]
        fn find_all_works() {
            let words_list = vec![
                "abcaaaa", "abdaa", "bca", "abc0010", "abc", "0abc", "abcabc",
            ];
            let prefix = "abc";
            let mut expected_result = vec!["abcaaaa", "abc0010", "abc", "abcabc"];
            expected_result.sort();

            let trie = CharTrie::from(words_list);

            let mut result = trie.find_all(prefix);
            result.sort();

            assert_eq!(result, expected_result);
        }
    }

    mod trie_size_is_correct {
        use super::*;

        #[rstest]
        fn empty_has_zero_len() {
            assert_returns!(0, CharTrie::len, &CharTrie::new());
        }

        #[rstest]
        fn top100trie_has_len_of_100(top100trie: CharTrie) {
            assert_returns!(100, CharTrie::len, &top100trie);
        }
    }

    proptest! {
        #[test]
        fn empty_trie_contains_nothing(ref word in ".*") {
            let empty_trie = CharTrie::new();

            assert!(!empty_trie.contains(word))
        }
    }
}

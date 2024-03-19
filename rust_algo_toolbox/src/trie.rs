pub mod hash_trie;
pub mod ternary_trie;

pub trait Trie {
    type Char;

    /// Insert an array of chars.
    /// Returns false if the word already exists
    fn insert(&mut self, word: impl Iterator<Item = Self::Char>) -> bool;

    /// Check if a word is in the trie
    fn contains(&self, word: impl Iterator<Item = Self::Char>) -> bool;

    /// Check if a prefix is in the trie
    fn contains_prefix(&self, prefix: impl Iterator<Item = Self::Char>) -> bool;

    /// Number of words in the trie
    fn len(&self) -> usize;

    /// Check if the trie is empty
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

pub trait StringSet {
    fn insert(&mut self, word: &str) -> bool;

    fn contains(&self, word: &str) -> bool;

    fn contains_prefix(&self, prefix: &str) -> bool;

    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<TrieImpl> StringSet for TrieImpl
where
    TrieImpl: Trie<Char = char>,
{
    fn insert(&mut self, word: &str) -> bool {
        self.insert(word.chars())
    }

    fn contains(&self, word: &str) -> bool {
        self.contains(word.chars())
    }

    fn contains_prefix(&self, prefix: &str) -> bool {
        self.contains_prefix(prefix.chars())
    }

    fn len(&self) -> usize {
        self.len()
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
    use std::fmt::Debug;

    use super::{hash_trie::HashTrie, ternary_trie::TernaryTrie, StringSet};

    trait PrintableTrie: StringSet + Debug {}
    impl<T> PrintableTrie for T where T: StringSet + Debug {}

    type DynStringSet = Box<dyn PrintableTrie>;

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
            let words_file_content = include_str!("../testdata/words100.txt");

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
    fn top100hash_trie(top100words: Vec<&str>) -> DynStringSet {
        let mut top100trie = HashTrie::default();
        for word in top100words {
            top100trie.insert(word);
        }
        Box::new(top100trie)
    }

    #[fixture]
    fn random_hash_trie(random_words: Vec<&str>) -> DynStringSet {
        let mut random_trie = HashTrie::default();
        for word in random_words {
            random_trie.insert(word);
        }
        Box::new(random_trie)
    }

    #[fixture]
    fn lol_kek_chebureck_hash_trie(lol_kek_chebureck_list: Vec<&str>) -> DynStringSet {
        let mut lol_kek_chebureck_trie = HashTrie::default();
        for word in lol_kek_chebureck_list {
            lol_kek_chebureck_trie.insert(word);
        }
        Box::new(lol_kek_chebureck_trie)
    }

    #[fixture]
    fn top100ternary_trie(top100words: Vec<&str>) -> DynStringSet {
        let mut top100trie = HashTrie::default();
        for word in top100words {
            top100trie.insert(word);
        }
        Box::new(top100trie)
    }

    #[fixture]
    fn random_ternary_trie(random_words: Vec<&str>) -> DynStringSet {
        let mut random_trie = HashTrie::default();
        for word in random_words {
            random_trie.insert(word);
        }
        Box::new(random_trie)
    }

    #[fixture]
    fn lol_kek_chebureck_ternary_trie(lol_kek_chebureck_list: Vec<&str>) -> DynStringSet {
        let mut lol_kek_chebureck_trie = HashTrie::default();
        for word in lol_kek_chebureck_list {
            lol_kek_chebureck_trie.insert(word);
        }
        Box::new(lol_kek_chebureck_trie)
    }

    ////////////////////////////////////////////////////////////////////////////////////

    mod trie_contains_inserted_words {
        use std::ops::Deref;

        use super::*;

        #[rstest]
        #[case(random_hash_trie(random_words()))]
        #[case(random_ternary_trie(random_words()))]
        fn random(#[case] random_trie: DynStringSet, random_words: Vec<&str>) {
            for word in random_words {
                //random_trie.contains(word);
                assert_returns!(true, StringSet::contains, random_trie.deref(), word);
            }
        }

        #[rstest]
        #[case(top100hash_trie(top100words()))]
        #[case(top100ternary_trie(top100words()))]
        fn top100(#[case] top100trie: DynStringSet, top100words: Vec<&str>) {
            for word in top100words {
                assert_returns!(true, StringSet::contains, top100trie.deref(), word);
            }
        }
    }

    mod trie_finds_inserted_words {
        use super::*;

        #[rstest]
        #[case(random_hash_trie(random_words()))]
        #[case(random_ternary_trie(random_words()))]
        fn random(#[case] random_trie: DynStringSet, random_words: Vec<&str>) {
            for word in random_words {
                assert_returns!(true, StringSet::contains, &*random_trie, word);
            }
        }

        #[rstest]
        #[case(top100hash_trie(top100words()))]
        #[case(top100ternary_trie(top100words()))]
        fn top100(#[case] top100trie: DynStringSet, top100words: Vec<&str>) {
            for word in top100words {
                assert_returns!(true, StringSet::contains, &*top100trie, word);
            }
        }
    }

    #[rstest]
    #[case(lol_kek_chebureck_hash_trie(lol_kek_chebureck_list()))]
    #[case(lol_kek_chebureck_ternary_trie(lol_kek_chebureck_list()))]
    fn insert_existing_returns_false(
        #[case] mut trie: DynStringSet,
        lol_kek_chebureck_list: Vec<&str>,
    ) {
        for word in lol_kek_chebureck_list {
            assert_returns!(false, StringSet::insert, &mut *trie, word);
        }
    }

    #[rstest]
    #[case(lol_kek_chebureck_hash_trie(lol_kek_chebureck_list()))]
    #[case(lol_kek_chebureck_ternary_trie(lol_kek_chebureck_list()))]
    fn insert_existing_doesnt_increase_size(
        #[case] mut trie: DynStringSet,
        lol_kek_chebureck_list: Vec<&str>,
    ) {
        for word in lol_kek_chebureck_list {
            trie.insert(word);
            assert_returns!(3, StringSet::len, &*trie);
        }
    }

    #[rstest]
    #[case(lol_kek_chebureck_hash_trie(lol_kek_chebureck_list()))]
    #[case(lol_kek_chebureck_ternary_trie(lol_kek_chebureck_list()))]
    fn contains_prefix_works(#[case] trie: DynStringSet, lol_kek_chebureck_list: Vec<&str>) {
        for word in lol_kek_chebureck_list {
            let incomplete_word = &word[0..word.len() - 1];
            assert_returns!(true, StringSet::contains_prefix, &*trie, incomplete_word);
        }
    }

    mod trie_size_is_correct {
        use super::*;

        #[rstest]
        #[case(Box::new(HashTrie::default()))]
        #[case(Box::new(TernaryTrie::default()))]
        fn empty_has_zero_len(#[case] trie: DynStringSet) {
            assert_returns!(0, StringSet::len, &*trie);
        }

        #[rstest]
        #[case(top100hash_trie(top100words()))]
        #[case(top100ternary_trie(top100words()))]
        fn top100trie_has_len_of_100(#[case] top100trie: DynStringSet) {
            assert_returns!(100, StringSet::len, &*top100trie);
        }
    }

    proptest! {
        #[test]
        fn empty_hash_trie_contains_nothing(ref word in ".*") {
            let empty_trie = HashTrie::new();

            assert!(!empty_trie.contains(word))
        }

        #[test]
        fn empty_ternary_trie_contains_nothing(ref word in ".*") {
            let empty_trie = TernaryTrie::new();

            assert!(!empty_trie.contains(word))
        }
    }
}

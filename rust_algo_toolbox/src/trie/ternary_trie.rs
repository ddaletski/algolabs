use super::Trie;
use std::{cmp::Ordering, fmt::Debug, iter::Peekable};

pub struct TernaryTrieNode<Char> {
    character: Char,
    word_end: bool,
    children: [Option<Box<TernaryTrieNode<Char>>>; 3],
}

impl<Char> TernaryTrieNode<Char>
where
    Char: Eq + Clone + Ord,
{
    fn new(character: Char, word_end: bool) -> Self {
        TernaryTrieNode {
            character,
            word_end,
            children: [None, None, None],
        }
    }

    fn insert(&mut self, word: impl Iterator<Item = Char>) -> bool {
        self.insert_impl(word.peekable())
    }

    fn contains(&self, word: impl Iterator<Item = Char>) -> bool {
        self.find_prefix_impl(word.peekable())
            .map(|node| node.word_end)
            .unwrap_or(false)
    }

    fn find_prefix(&self, prefix: impl Iterator<Item = Char>) -> Option<&Self> {
        self.find_prefix_impl(prefix.peekable())
    }

    fn insert_impl(&mut self, mut word: Peekable<impl Iterator<Item = Char>>) -> bool {
        let Some(next_char) = word.peek().cloned() else {
            if self.word_end {
                return false;
            } else {
                self.word_end = true;
                return true;
            }
        };

        let child_idx = match next_char.cmp(&self.character) {
            Ordering::Less => 0,
            Ordering::Equal => {
                word.next();
                1
            }
            Ordering::Greater => 2,
        };

        self.children[child_idx]
            .get_or_insert(Box::new(TernaryTrieNode::new(next_char.clone(), false)))
            .insert_impl(word)
    }

    fn find_prefix_impl(&self, mut word: Peekable<impl Iterator<Item = Char>>) -> Option<&Self> {
        let Some(next_char) = word.peek().cloned() else {
            return Some(self);
        };

        let child_idx = match next_char.cmp(&self.character) {
            Ordering::Less => 0,
            Ordering::Equal => {
                word.next();
                1
            }
            Ordering::Greater => 2,
        };

        let Some(child) = &self.children[child_idx] else {
            return None;
        };

        child.find_prefix_impl(word)
    }
}

impl<Char> TernaryTrieNode<Char>
where
    Char: Debug,
{
    fn format_impl(&self, indent: usize, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let space = "| ".repeat(indent);
        f.write_fmt(format_args!("{}{:?}\n", space, self.character))?;

        for node in &self.children {
            if let Some(node) = node {
                node.format_impl(indent + 1, f)?;
            }
        }

        Ok(())
    }
}

impl<Char: Debug> Debug for TernaryTrieNode<Char> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.format_impl(0, f)?;
        Ok(())
    }
}

pub struct TernaryTrie<Char> {
    root: TernaryTrieNode<Char>,
    words_count: usize,
}

impl<Char> TernaryTrie<Char>
where
    Char: Default + Clone + Ord,
{
    pub fn new() -> Self {
        TernaryTrie {
            root: TernaryTrieNode::new(Char::default(), false),
            words_count: 0,
        }
    }
}

impl<Char> Trie for TernaryTrie<Char>
where
    Char: Ord + Clone,
{
    type Char = Char;

    fn insert(&mut self, word: impl Iterator<Item = Self::Char>) -> bool {
        let result = self.root.insert(word);

        if result {
            self.words_count += 1;
        }

        result
    }

    fn contains(&self, word: impl Iterator<Item = Self::Char>) -> bool {
        self.root.contains(word)
    }

    fn contains_prefix(&self, prefix: impl Iterator<Item = Self::Char>) -> bool {
        self.root.find_prefix(prefix).is_some()
    }

    fn len(&self) -> usize {
        self.words_count
    }
}

impl<Char> Default for TernaryTrie<Char>
where
    Char: Clone + Default + Ord,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<Char: Debug> Debug for TernaryTrie<Char> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.root.fmt(f)?;
        Ok(())
    }
}

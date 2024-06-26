use std::{collections::HashMap, fmt::Debug, hash::Hash, ops::Deref};

use super::Trie;

pub struct HashTrieNode<Char> {
    character: Char,
    word_end: bool,
    children: HashMap<Char, Box<HashTrieNode<Char>>>,
}

impl<Char> HashTrieNode<Char>
where
    Char: Eq + Hash + Clone,
{
    fn new(character: Char, word_end: bool) -> Self {
        HashTrieNode {
            character,
            word_end,
            children: HashMap::new(),
        }
    }

    fn insert(&mut self, word: impl Iterator<Item = Char>) -> bool {
        self.insert_impl(word)
    }

    pub fn find_prefix(&self, mut prefix: impl Iterator<Item = Char>) -> Option<&Self> {
        if let Some(next_char) = prefix.next() {
            if let Some(next_node) = self.children.get(&next_char) {
                return next_node.deref().find_prefix(prefix);
            }
            return None;
        }
        Some(self)
    }

    fn insert_impl(&mut self, mut word: impl Iterator<Item = Char>) -> bool {
        if let Some(next_char) = word.next() {
            match &mut self.children.get_mut(&next_char) {
                Some(next_node) => {
                    return next_node.insert_impl(word);
                }
                None => {
                    let mut next_node = Box::new(HashTrieNode::new(next_char.clone(), false));
                    let result = next_node.insert_impl(word);

                    self.children.insert(next_char, next_node);
                    return result;
                }
            }
        }

        if self.word_end {
            return false;
        }

        self.word_end = true;
        true
    }

    pub fn word_end(&self) -> bool {
        self.word_end
    }

    pub fn next(&self, character: Char) -> Option<&HashTrieNode<Char>> {
        self.children.get(&character).map(|node| node.deref())
    }
}

impl<Char> HashTrieNode<Char>
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

impl<Char: Debug> Debug for HashTrieNode<Char> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.format_impl(0, f)?;
        Ok(())
    }
}

pub struct HashTrie<Char> {
    root: HashTrieNode<Char>,
    words_count: usize,
}

impl<Char> HashTrie<Char>
where
    Char: Default + Clone + Hash + Eq,
{
    pub fn new() -> Self {
        HashTrie {
            root: HashTrieNode::new(Char::default(), false),
            words_count: 0,
        }
    }

    pub fn root(&self) -> &HashTrieNode<Char> {
        &self.root
    }

    pub fn find_all(&self, prefix: impl Iterator<Item = Char>) -> Vec<Vec<Char>> {
        let mut result = vec![];
        let prefix_node = self.root.find_prefix(prefix);

        let Some(prefix_node) = prefix_node else {
            return result;
        };

        let mut prefix_chars = vec![];
        prefix_chars.push(prefix_node.character.clone());
        self.find_all_impl(prefix_node, &mut prefix_chars, &mut result);

        result
    }

    fn find_all_impl(
        &self,
        node: &HashTrieNode<Char>,
        prefix_chars: &mut Vec<Char>,
        result: &mut Vec<Vec<Char>>,
    ) {
        if node.word_end {
            result.push(prefix_chars.clone());
        }

        for child in node.children.values() {
            prefix_chars.push(child.character.clone());
            self.find_all_impl(child, prefix_chars, result);
            prefix_chars.pop();
        }
    }
}

impl<Char> Trie for HashTrie<Char>
where
    Char: Eq + Hash + Clone,
{
    type Char = Char;

    fn insert(&mut self, word: impl Iterator<Item = Char>) -> bool {
        let inserted = self.root.insert(word);

        if inserted {
            self.words_count += 1;
        }

        inserted
    }

    fn contains(&self, word: impl Iterator<Item = Char>) -> bool {
        self.root
            .find_prefix(word)
            .map(|node| node.word_end)
            .unwrap_or(false)
    }

    fn contains_prefix(&self, prefix: impl Iterator<Item = Self::Char>) -> bool {
        self.root.find_prefix(prefix).is_some()
    }

    fn len(&self) -> usize {
        self.words_count
    }
}

impl<Char> Default for HashTrie<Char>
where
    Char: Eq + Hash + Clone + Debug + Default,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<Char: Debug> Debug for HashTrie<Char> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.root.fmt(f)?;
        Ok(())
    }
}

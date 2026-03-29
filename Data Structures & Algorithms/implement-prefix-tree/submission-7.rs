use std::collections::HashMap;

struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_word: bool,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            children: HashMap::new(),
            is_word: false,
        }
    }
}

pub struct PrefixTree {
    root: TrieNode,
}

impl PrefixTree {
    pub fn new() -> Self {
        PrefixTree {
            root: TrieNode::new(),
        }
    }

    pub fn insert(&mut self, word: String) {
        let mut curr = &mut self.root;
        for c in word.chars() {
            curr = curr.children.entry(c).or_insert(TrieNode::new());
        }
        curr.is_word = true;
    }

    pub fn search(&self, word: String) -> bool {
        let mut curr = &self.root;
        for c in word.chars() {
            if let Some(node) = curr.children.get(&c) {
                curr = node;
            } else {
                return false;
            }
        }
        curr.is_word
    }

    pub fn starts_with(&self, prefix: String) -> bool {
        let mut curr = &self.root;
        for c in prefix.chars() {
            if let Some(node) = curr.children.get(&c) {
                curr = node;
            } else {
                return false;
            }
        }
        true
    }
}
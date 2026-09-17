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

struct Trie {
    root: TrieNode,
}

impl Trie {
    fn new() -> Self {
        Trie {
            root: TrieNode::new(),
        }
    }

    fn insert(&mut self, word: String) {
        let mut curr = &mut self.root;
        for c in word.chars() {
            curr = curr.children.entry(c).or_insert(TrieNode::new());
        }
        curr.is_word = true;
    }

    fn search(&self, word: String) -> bool {
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

    fn starts_with(&self, prefix: String) -> bool {
        let mut curr = &self.root;
        for c in prefix.chars() {
            if let Some(node) = curr.children.get(&c) {
                curr = node;
            } else {
                return false;
            }
        }
        true;
    }
}
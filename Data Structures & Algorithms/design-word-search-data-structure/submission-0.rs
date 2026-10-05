#[derive(Default)]
struct TrieNode {
    children: [Option<Box<TrieNode>>; 26],
    is_word: bool,
}

pub struct WordDictionary {
    root: TrieNode,
}

impl WordDictionary {
    pub fn new() -> Self {
        WordDictionary {
            root: TrieNode::default(),
        }
    }

    pub fn add_word(&mut self, word: String) {
        let mut curr = &mut self.root;
        for c in word.bytes() {
            let idx = (c - b'a') as usize;
            curr = curr.children[idx].get_or_insert_with(|| Box::new(TrieNode::default()));
        }
        curr.is_word = true;
    }

    pub fn search(&self, word: String) -> bool {
        Self::dfs(word.as_bytes(), 0, &self.root)
    }

    fn dfs(word: &[u8], index: usize, node: &TrieNode) -> bool {
        if index == word.len() {
            return node.is_word;
        }

        let c = word[index];
        
        if c == b'.' {
            for child in node.children.iter().flatten() {
                if Self::dfs(word, index + 1, child) {
                    return true;
                }
            }
            false
        } else {
            let idx = (c - b'a') as usize;
            if let Some(child) = &node.children[idx] {
                Self::dfs(word, index + 1, child)
            } else {
                false
            }
        }
    }
}
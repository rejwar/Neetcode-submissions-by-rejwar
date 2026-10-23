#[derive(Default)]
struct TrieNode {
    children: [Option<Box<TrieNode>>; 26],
    count: i32,
}

#[derive(Default)]
struct Trie {
    root: TrieNode,
}

impl Trie {
    fn insert(&mut self, word: &str) {
        let mut curr = &mut self.root;
        for b in word.bytes() {
            let idx = (b - b'a') as usize;
            let next = curr.children[idx].get_or_insert_with(|| Box::new(TrieNode::default()));
            next.count += 1;
            curr = next;
        }
    }

    fn get_score(&self, word: &str) -> i32 {
        let mut curr = &self.root;
        let mut score = 0;
        for b in word.bytes() {
            let idx = (b - b'a') as usize;
            if let Some(next) = &curr.children[idx] {
                score += next.count;
                curr = next;
            }
        }
        score
    }
}

impl Solution {
    pub fn sum_prefix_scores(words: Vec<String>) -> Vec<i32> {
        let mut trie = Trie::default();
        
        // Build the Trie with prefix counts
        for word in &words {
            trie.insert(word);
        }
        
        // Calculate the score for each word
        words.iter().map(|word| trie.get_score(word)).collect()
    }
}
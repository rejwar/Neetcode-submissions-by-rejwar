#[derive(Default)]
struct TrieNode {
    children: [Option<usize>; 26],
    word: Option<String>,
}

pub struct Trie {
    nodes: Vec<TrieNode>,
}

impl Trie {
    fn new() -> Self {
        Self {
            nodes: vec![TrieNode::default()],
        }
    }

    fn insert(&mut self, word: String) {
        let mut curr = 0;
        for &b in word.as_bytes() {
            let idx = (b - b'a') as usize;
            if self.nodes[curr].children[idx].is_none() {
                let next_idx = self.nodes.len();
                self.nodes.push(TrieNode::default());
                self.nodes[curr].children[idx] = Some(next_idx);
            }
            curr = self.nodes[curr].children[idx].unwrap();
        }
        self.nodes[curr].word = Some(word);
    }
}

impl Solution {
    pub fn find_words(mut board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        let mut trie = Trie::new();
        for word in words {
            trie.insert(word);
        }

        let rows = board.len();
        let cols = board[0].len();
        let mut result = Vec::new();

        for r in 0..rows {
            for c in 0..cols {
                Self::dfs(&mut board, r, c, 0, &mut trie, &mut result);
            }
        }

        result
    }

    fn dfs(
        board: &mut Vec<Vec<char>>,
        r: usize,
        c: usize,
        node_idx: usize,
        trie: &mut Trie,
        result: &mut Vec<String>,
    ) {
        let ch = board[r][c];
        if ch == '#' {
            return;
        }

        let char_idx = (ch as u8 - b'a') as usize;
        let next_node_idx = match trie.nodes[node_idx].children[char_idx] {
            Some(idx) => idx,
            None => return,
        };

        if let Some(word) = trie.nodes[next_node_idx].word.take() {
            result.push(word);
        }

        board[r][c] = '#';

        let rows = board.len();
        let cols = board[0].len();
        let dirs: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

        for (dr, dc) in dirs {
            let nr = r as isize + dr;
            let nc = c as isize + dc;
            if nr >= 0 && nr < rows as isize && nc >= 0 && nc < cols as isize {
                Self::dfs(board, nr as usize, nc as usize, next_node_idx, trie, result);
            }
        }

        board[r][c] = ch;
    }
}
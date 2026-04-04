impl Solution {
    pub fn foreign_dictionary(words: Vec<String>) -> String {
        let mut adj: HashMap<u8, HashSet<u8>> = HashMap::new();
        let mut indegree: HashMap<u8, i32> = HashMap::new();

        for word in &words {
            for &c in word.as_bytes() {
                adj.entry(c).or_insert_with(HashSet::new);
                indegree.entry(c).or_insert(0);
            }
        }

        for i in 0..words.len() - 1 {
            let w1 = words[i].as_bytes();
            let w2 = words[i + 1].as_bytes();
            let min_len = w1.len().min(w2.len());
            if w1.len() > w2.len() && w1[..min_len] == w2[..min_len] {
                return String::new();
            }
            for j in 0..min_len {
                if w1[j] != w2[j] {
                    if adj.get_mut(&w1[j]).unwrap().insert(w2[j]) {
                        *indegree.get_mut(&w2[j]).unwrap() += 1;
                    }
                    break;
                }
            }
        }

        let mut q: VecDeque<u8> = VecDeque::new();
        for (&c, &deg) in &indegree {
            if deg == 0 {
                q.push_back(c);
            }
        }

        let mut res: Vec<u8> = Vec::new();
        while let Some(ch) = q.pop_front() {
            res.push(ch);
            if let Some(neighbors) = adj.get(&ch) {
                for &neighbor in neighbors {
                    let deg = indegree.get_mut(&neighbor).unwrap();
                    *deg -= 1;
                    if *deg == 0 {
                        q.push_back(neighbor);
                    }
                }
            }
        }

        if res.len() != indegree.len() {
            return String::new();
        }

        String::from_utf8(res).unwrap()
    }
}
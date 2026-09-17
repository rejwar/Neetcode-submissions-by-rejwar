use std::collections::{HashMap, HashSet, VecDeque};

impl Solution {
    pub fn alien_order(words: Vec<String>) -> String {
        let mut adj: HashMap<char, HashSet<char>> = HashMap::new();
        let mut count: HashMap<char, i32> = HashMap::new();

        // 
        for word in &words {
            for c in word.chars() {
                count.entry(c).or_insert(0);
            }
        }

        //
        for i in 0..words.len() - 1 {
            let w1: Vec<char> = words[i].chars().collect();
            let w2: Vec<char> = words[i + 1].chars().collect();
            let min_len = std::cmp::min(w1.len(), w2.len());

            
            if w1.len() > w2.len() && w1[..min_len] == w2[..min_len] {
                return "".to_string();
            }

            for j in 0..min_len {
                if w1[j] != w2[j] {
                    if adj.entry(w1[j]).or_default().insert(w2[j]) {
                        *count.entry(w2[j]).or_default() += 1;
                    }
                    break;
                }
            }
        }

        
        let mut queue: VecDeque<char> = count
            .iter()
            .filter(|&(_, &v)| v == 0)
            .map(|(&k, _)| k)
            .collect();

        let mut res = String::new();
        while let Some(u) = queue.pop_front() {
            res.push(u);
            if let Some(neighbors) = adj.get(&u) {
                for &v in neighbors {
                    let entry = count.get_mut(&v).unwrap();
                    *entry -= 1;
                    if *entry == 0 {
                        queue.push_back(v);
                    }
                }
            }
        }

        
        if res.len() == count.len() { res } else { "".to_string() }
    }
}
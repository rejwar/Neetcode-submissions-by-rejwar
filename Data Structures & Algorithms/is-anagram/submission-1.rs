use std::collections::HashMap;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut count = HashMap::new();

        for c in s.chars() {
            *count.entry(c).or_insert(0) += 1;
        }

        for c in t.chars() {
            let val = count.entry(c).or_insert(0);
            if *val == 0 {
                return false;
            }
            *val -= 1;
        }

        true
    }
}
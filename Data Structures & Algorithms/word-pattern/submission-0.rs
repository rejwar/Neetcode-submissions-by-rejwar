use std::collections::HashMap;

impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        let words: Vec<&str> = s.split_whitespace().collect();
        
        if pattern.len() != words.len() {
            return false;
        }

        let mut char_to_word = HashMap::new();
        let mut word_to_char = HashMap::new();

        for (c, w) in pattern.chars().zip(words) {
            if char_to_word.insert(c, w).unwrap_or(w) != w {
                return false;
            }
            if word_to_char.insert(w, c).unwrap_or(c) != c {
                return false;
            }
        }

        true
    }
}
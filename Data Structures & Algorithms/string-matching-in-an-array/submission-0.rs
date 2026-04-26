impl Solution {
    pub fn string_matching(words: Vec<String>) -> Vec<String> {
        words.iter()
            .filter(|&w1| words.iter().any(|w2| w1 != w2 && w2.contains(w1)))
            .cloned()
            .collect()
    }
}
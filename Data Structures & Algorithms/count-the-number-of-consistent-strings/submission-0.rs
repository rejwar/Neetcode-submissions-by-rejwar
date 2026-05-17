impl Solution {
    pub fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
        let mut allowed_set = [false; 26];
        for b in allowed.bytes() {
            allowed_set[(b - b'a') as usize] = true;
        }
        
        words.iter()
            .filter(|word| word.bytes().all(|b| allowed_set[(b - b'a') as usize]))
            .count() as i32
    }
}
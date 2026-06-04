impl Solution {
    pub fn make_equal(words: Vec<String>) -> bool {
        let n = words.len();
        let mut counts = [0; 26];
        
        for word in &words {
            for b in word.bytes() {
                counts[(b - b'a') as usize] += 1;
            }
        }
        
        counts.into_iter().all(|count| count % n == 0)
    }
}
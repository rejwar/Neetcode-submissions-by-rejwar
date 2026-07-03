impl Solution {
    pub fn count_prefix_suffix_pairs(words: Vec<String>) -> i32 {
        let mut count = 0;
        let n = words.len();
        
        for i in 0..n {
            for j in (i + 1)..n {
                // Check if words[i] is both a prefix and a suffix of words[j]
                if words[j].starts_with(&words[i]) && words[j].ends_with(&words[i]) {
                    count += 1;
                }
            }
        }
        
        count
    }
}
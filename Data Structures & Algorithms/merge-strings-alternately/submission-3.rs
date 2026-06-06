impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let b1 = word1.as_bytes();
        let b2 = word2.as_bytes();
        let mut result = Vec::with_capacity(word1.len() + word2.len());
        
        // Merge characters alternately until one string runs out
        let min_len = b1.len().min(b2.len());
        for i in 0..min_len {
            result.push(b1[i]);
            result.push(b2[i]);
        }
        
        // Append any remaining characters from either string.
        // One of these slices will be empty, so both calls are safe and efficient.
        result.extend_from_slice(&b1[min_len..]);
        result.extend_from_slice(&b2[min_len..]);
        
        String::from_utf8(result)
    }
}
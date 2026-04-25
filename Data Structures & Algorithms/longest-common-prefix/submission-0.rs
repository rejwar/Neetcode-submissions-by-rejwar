impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.is_empty() { return String::new(); }
        
        let mut prefix = strs[0].clone();
        
        for s in strs.iter().skip(1) {
            while !s.starts_with(&prefix) {
                prefix.pop();
            }
        }
        
        prefix
    }
}
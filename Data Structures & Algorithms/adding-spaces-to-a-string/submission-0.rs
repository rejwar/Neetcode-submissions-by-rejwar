impl Solution {
    pub fn add_spaces(s: String, spaces: Vec<i32>) -> String {
        let bytes = s.as_bytes();
        let mut result = String::with_capacity(s.len() + spaces.len());
        let mut space_idx = 0;
        
        for (i, &b) in bytes.iter().enumerate() {
            if space_idx < spaces.len() && i == spaces[space_idx] as usize {
                result.push(' ');
                space_idx += 1;
            }
            result.push(b as char);
        }
        
        result
    }
}
impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let mut s_chars = s.chars();
        let mut expected_char = s_chars.next();

        for c in t.chars() {
            if Some(c) == expected_char {
                expected_char = s_chars.next(); 
            }
        }
        
        //  expected_char None 
        expected_char.is_none()
    }
}
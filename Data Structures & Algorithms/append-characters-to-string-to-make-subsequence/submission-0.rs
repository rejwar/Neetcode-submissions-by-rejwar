impl Solution {
    pub fn append_characters(s: String, t: String) -> i32 {
        let s_bytes = s.as_bytes();
        let t_bytes = t.as_bytes();
        
        let mut j = 0; 

       
        for &c in s_bytes {
           
            if j < t_bytes.len() && c == t_bytes[j] {
                j += 1; 
            }
        }

        (t_bytes.len() - j) as i32
    
}
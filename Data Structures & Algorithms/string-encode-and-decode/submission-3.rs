impl Solution {
    pub fn encode(strs: Vec<String>) -> String {
        let mut encoded = String::new();
        for s in strs {
            encoded.push_str(&format!("{}#{}", s.len(), s));
        }
        encoded
    }

    pub fn decode(s: String) -> Vec<String> {
        let mut res = Vec::new();
        let mut i = 0;
        let bytes = s.as_bytes();

        while i < bytes.len() {
            let mut j = i;
            while j < bytes.len() && bytes[j] != b'#' {
                j += 1;
            }
            
            let len: usize = s[i..j].parse().unwrap();
            i = j + 1; 
            
            let str_val = s[i..i + len].to_string();
            res.push(str_val);
            i += len; 
        }
        res
    }
}
impl Solution {
    pub fn reverse_parentheses(s: String) -> String {
        let bytes = s.as_bytes();
        let n = bytes.len();
        let mut pair = vec![0; n];
        let mut stack = Vec::new();
        
        for i in 0..n {
            if bytes[i] == b'(' {
                stack.push(i);
            } else if bytes[i] == b')' {
                if let Some(j) = stack.pop() {
                    pair[i] = j;
                    pair[j] = i;
                }
            }
        }
        
        let mut res = String::new();
        let mut i: i32 = 0;
        let mut d: i32 = 1;
        
        while i >= 0 && (i as usize) < n {
            let idx = i as usize;
            if bytes[idx] == b'(' || bytes[idx] == b')' {
                i = pair[idx] as i32;
                d = -d;
            } else {
                res.push(bytes[idx] as char);
            }
            i += d;
        }
        
        res
    }
}
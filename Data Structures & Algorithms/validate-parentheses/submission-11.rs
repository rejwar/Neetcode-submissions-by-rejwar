impl Solution {
    pub fn reverse_parentheses(s: String) -> String {
        let s = s.as_bytes();
        let n = s.len();
        let mut pair = vec![0; n];
        let mut stack = Vec::new();
        
        for i in 0..n {
            if s[i] == b'(' {
                stack.push(i);
            } else if s[i] == b')' {
                let j = stack.pop().unwrap();
                pair[i] = j;
                pair[j] = i;
            }
        }
        
        let mut res = String::with_capacity(n);
        let mut i = 0;
        let mut d: i32 = 1;
        
        while i < n {
            if s[i] == b'(' || s[i] == b')' {
                i = pair[i];
                d = -d;
            } else {
                res.push(s[i] as char);
            }
            i = (i as i32 + d) as usize;
        }
        
        res
    }
}
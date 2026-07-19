impl Solution {
    pub fn calculate(s: String) -> i32 {
        let mut stack = Vec::new();
        let mut num = 0;
        let mut sign = b'+';
        
        let s = s.as_bytes();
        for i in 0..s.len() {
            let c = s[i];
            if c.is_ascii_digit() {
                num = num * 10 + (c - b'0') as i32;
            }
            if (!c.is_ascii_digit() && c != b' ') || i == s.len() - 1 {
                match sign {
                    b'+' => stack.push(num),
                    b'-' => stack.push(-num),
                    b'*' => *stack.last_mut().unwrap() *= num,
                    b'/' => *stack.last_mut().unwrap() /= num,
                    _ => unreachable!(),
                }
                sign = c;
                num = 0;
            }
        }
        stack.iter().sum()
    }
}
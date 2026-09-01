impl Solution {
    pub fn decode_string(s: String) -> String {
        let mut stack = Vec::new();
        let mut curr_str = String::new();
        let mut curr_num = 0;

        for c in s.chars() {
            if c.is_ascii_digit() {
                curr_num = curr_num * 10 + c.to_digit(10).unwrap() as usize;
            } else if c == '[' {
                stack.push((curr_str, curr_num));
                curr_str = String::new();
                curr_num = 0;
            } else if c == ']' {
                if let Some((mut prev_str, num)) = stack.pop() {
                    prev_str.push_str(&curr_str.repeat(num));
                    curr_str = prev_str;
                }
            } else {
                curr_str.push(c);
            }
        }
        
        curr_str
    }
}
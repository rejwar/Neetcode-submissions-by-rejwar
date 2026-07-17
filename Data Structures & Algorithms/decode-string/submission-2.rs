impl Solution {
    pub fn decode_string(s: String) -> String {
        let mut stack: Vec<(String, u32)> = Vec::new();
        let mut current_string = String::new();
        let mut current_num = 0;

        for c in s.chars() {
            if c.is_ascii_digit() {
                current_num = current_num * 10 + c.to_digit(10).unwrap();
            } else if c == '[' {
                stack.push((current_string, current_num));
                current_string = String::new();
                current_num = 0;
            } else if c == ']' {
                if let Some((prev_string, num)) = stack.pop() {
                    current_string = prev_string + &current_string.repeat(num as usize);
                }
            } else {
                current_string.push(c);
            }
        }

        current_string
    }
}
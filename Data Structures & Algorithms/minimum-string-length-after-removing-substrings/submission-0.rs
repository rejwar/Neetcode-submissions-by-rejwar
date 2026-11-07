impl Solution {
    pub fn min_length(s: String) -> i32 {
        let mut stack: Vec<char> = Vec::with_capacity(s.len());

        for c in s.chars() {
            match (stack.last(), c) {
                (Some(&'A'), 'B') | (Some(&'C'), 'D') => {
                    stack.pop();
                }
                _ => {
                    stack.push(c);
                }
            }
        }

        stack.len() as i32
    }
}
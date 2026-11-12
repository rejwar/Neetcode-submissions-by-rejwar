impl Solution {
    pub fn backspace_compare(s: String, t: String) -> bool {
        let process = |input: &str| -> String {
            let mut stack = String::new();
            for c in input.chars() {
                if c == '#' {
                    stack.pop();
                } else {
                    stack.push(c);
                }
            }
            stack
        };

        process(&s) == process(&t)
    }
}
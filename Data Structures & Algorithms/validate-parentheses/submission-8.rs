impl Solution {
    pub fn reverse_parentheses(s: String) -> String {
        let mut stack: Vec<String> = vec![String::new()];
        
        for c in s.chars() {
            if c == '(' {
                stack.push(String::new());
            } else if c == ')' {
                let reversed: String = stack.pop().unwrap().chars().rev().collect();
                stack.last_mut().unwrap().push_str(&reversed);
            } else {
                stack.last_mut().unwrap().push(c);
            }
        }
        
        stack.pop().unwrap()
    }
}
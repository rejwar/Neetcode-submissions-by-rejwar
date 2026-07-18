impl Solution {
    pub fn remove_duplicates(s: String, k: i32) -> String {
        let mut stack: Vec<(char, i32)> = Vec::new();
        
        for c in s.chars() {
            if let Some(last) = stack.last_mut() {
                if last.0 == c {
                    last.1 += 1;
                    if last.1 == k {
                        stack.pop();
                    }
                    continue;
                }
            }
            stack.push((c, 1));
        }
        
        let mut result = String::with_capacity(s.len());
        for (c, count) in stack {
            for _ in 0..count {
                result.push(c);
            }
        }
        
        result
    }
}
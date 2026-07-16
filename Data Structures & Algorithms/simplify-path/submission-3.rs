impl Solution {
    pub fn simplify_path(path: String) -> String {
        let mut stack = Vec::new();
        
        for part in path.split('/') {
            match part {
                "" | "." => {}
                ".." => {
                    stack.pop();
                }
                _ => {
                    stack.push(part);
                }
            }
        }
        
        format!("/{}", stack.join("/"))
    }
}
impl Solution {
    pub fn min_operations(logs: Vec<String>) -> i32 {
        logs.iter().fold(0, |depth, log| match log.as_str() {
            "../" => 0.max(depth - 1),
            "./" => depth,
            _ => depth + 1,
        })
    }
}
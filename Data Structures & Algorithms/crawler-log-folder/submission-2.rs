impl Solution {
    pub fn min_operations(logs: Vec<String>) -> i32 {
        logs.iter().fold(0, |depth, log| {
            match log.as_str() {
                "../" => (depth - 1).max(0),
                "./" => depth,
                _ => depth + 1,
            }
        })
    }
}
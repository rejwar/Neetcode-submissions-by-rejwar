impl Solution {
    pub fn count_seniors(details: Vec<String>) -> i32 {
        details.iter()
            .filter(|d| &d[11..13] > "60")
            .count() as i32
    }
}
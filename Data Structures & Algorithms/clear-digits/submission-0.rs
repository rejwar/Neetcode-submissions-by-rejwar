impl Solution {
    pub fn clear_digits(s: String) -> String {
        let mut res = String::with_capacity(s.len());
        for c in s.chars() {
            if c.is_ascii_digit() {
                res.pop();
            } else {
                res.push(c);
            }
        }
        res
    }
}
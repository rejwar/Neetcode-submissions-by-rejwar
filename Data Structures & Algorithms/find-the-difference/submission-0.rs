impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char {
        s.bytes()
            .chain(t.bytes())
            .fold(0, |acc, b| acc ^ b) as char
    }
}
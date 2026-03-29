impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let mut chars = s.chars()
            .filter(|c| c.is_alphanumeric())
            .map(|c| c.to_ascii_lowercase());

        while let (Some(left), Some(right)) = (chars.next(), chars.next_back()) {
            if left != right {
                return false;
            }
        }
        true
    }
}
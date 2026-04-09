impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let it = s.chars()
            .filter(|c| c.is_alphanumeric())
            .map(|c| c.to_ascii_lowercase());

        it.clone().eq(it.rev())
    }
}
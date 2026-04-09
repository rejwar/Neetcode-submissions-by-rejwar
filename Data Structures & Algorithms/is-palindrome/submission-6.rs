

impl Solution {
    pub fn is_palindrome(s: String) ->bool {
        let clean = s.chars()
        .filter(|c|c.is_alphanumeric())
        .map(|c|c.to_ascii_lowercase());

        clean.clone().eq(clean.rev())
    }
}
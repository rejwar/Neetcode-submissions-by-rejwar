impl Solution {
    pub fn first_palindrome(words: Vec<String>) -> String {
        words
            .into_iter()
            .find(|w| w.as_bytes().iter().eq(w.as_bytes().iter().rev()))
            .unwrap_or_default()
    }
}
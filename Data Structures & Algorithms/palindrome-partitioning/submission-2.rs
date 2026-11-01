impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let mut res = Vec::new();
        let mut current = Vec::new();
        let bytes = s.as_bytes();
        
        Self::backtrack(&s, bytes, 0, &mut current, &mut res);
        res
    }

    fn backtrack(
        s: &str,
        bytes: &[u8],
        start: usize,
        current: &mut Vec<String>,
        res: &mut Vec<Vec<String>>,
    ) {
        if start == bytes.len() {
            res.push(current.clone());
            return;
        }

        for end in start..bytes.len() {
            if Self::is_palindrome(bytes, start, end) {
                current.push(s[start..=end].to_string());
                Self::backtrack(s, bytes, end + 1, current, res);
                current.pop();
            }
        }
    }

    fn is_palindrome(bytes: &[u8], mut left: usize, mut right: usize) -> bool {
        while left < right {
            if bytes[left] != bytes[right] {
                return false;
            }
            left += 1;
            right -= 1;
        }
        true
    }
}
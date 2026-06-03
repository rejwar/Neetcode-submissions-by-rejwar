impl Solution {
    pub fn valid_palindrome(s: String) -> bool {
        let bytes = s.as_bytes();
        let mut left = 0;
        let mut right = bytes.len().saturating_sub(1);

        while left < right {
            if bytes[left] != bytes[right] {
                // If a mismatch is found, we can either skip the left character 
                // or the right character. Check if either resulting substring is a palindrome.
                return Self::is_palindrome(bytes, left + 1, right) || 
                       Self::is_palindrome(bytes, left, right - 1);
            }
            left += 1;
            right -= 1;
        }

        true
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
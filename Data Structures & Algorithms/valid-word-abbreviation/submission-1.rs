impl Solution {
    pub fn valid_word_abbreviation(word: String, abbr: String) -> bool {
        let w_bytes = word.as_bytes();
        let a_bytes = abbr.as_bytes();
        let mut i = 0;
        let mut j = 0;

        while i < w_bytes.len() && j < a_bytes.len() {
            if a_bytes[j].is_ascii_digit() {
                // Leading zeros are invalid
                if a_bytes[j] == b'0' {
                    return false;
                }
                
                let mut num = 0;
                while j < a_bytes.len() && a_bytes[j].is_ascii_digit() {
                    num = num * 10 + (a_bytes[j] - b'0') as usize;
                    j += 1;
                }
                
                i += num;
            } else {
                if w_bytes[i] != a_bytes[j] {
                    return false;
                }
                i += 1;
                j += 1;
            }
        }

        i == w_bytes.len() && j == a_bytes.len()
    }
}
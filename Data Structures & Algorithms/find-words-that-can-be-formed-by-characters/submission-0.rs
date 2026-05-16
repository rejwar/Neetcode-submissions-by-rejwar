impl Solution {
    pub fn count_characters(words: Vec<String>, chars: String) -> i32 {
        let mut char_counts = [0; 26];
        for b in chars.bytes() {
            char_counts[(b - b'a') as usize] += 1;
        }

        let mut total_length = 0;

        for word in words {
            let mut word_counts = [0; 26];
            let mut is_good = true;
            
            for b in word.bytes() {
                let idx = (b - b'a') as usize;
                word_counts[idx] += 1;
                if word_counts[idx] > char_counts[idx] {
                    is_good = false;
                    break;
                }
            }

            if is_good {
                total_length += word.len() as i32;
            }
        }

        total_length
    }
}
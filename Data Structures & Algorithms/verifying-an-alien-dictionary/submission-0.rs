impl Solution {
    pub fn is_alien_sorted(words: Vec<String>, order: String) -> bool {
        let mut char_map = [0; 26];
        for (i, c) in order.chars().enumerate() {
            char_map[(c as u8 - b'a') as usize] = i;
        }

        for i in 0..words.len() - 1 {
            let w1 = words[i].as_bytes();
            let w2 = words[i + 1].as_bytes();
            let mut sorted = false;

            for j in 0..std::cmp::min(w1.len(), w2.len()) {
                let char1 = (w1[j] - b'a') as usize;
                let char2 = (w2[j] - b'a') as usize;

                if char_map[char1] < char_map[char2] {
                    sorted = true;
                    break;
                } else if char_map[char1] > char_map[char2] {
                    return false;
                }
            }

            if !sorted && w1.len() > w2.len() {
                return false;
            }
        }

        true
    }
}
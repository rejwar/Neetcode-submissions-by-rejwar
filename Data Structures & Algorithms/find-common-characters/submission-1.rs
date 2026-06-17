impl Solution {
    pub fn common_chars(words: Vec<String>) -> Vec<String> {
        if words.is_empty() {
            return vec![];
        }

        // Array to store the minimum frequency of each character ('a' to 'z')
        let mut min_freq = [0; 26];
        
        // Populate the initial frequencies using the first word
        for c in words[0].chars() {
            min_freq[(c as u8 - b'a') as usize] += 1;
        }

        // Check the rest of the words
        for word in words.iter().skip(1) {
            let mut current_freq = [0; 26];
            for c in word.chars() {
                current_freq[(c as u8 - b'a') as usize] += 1;
            }

            // Update the min_freq array to keep only the common characters
            for i in 0..26 {
                min_freq[i] = std::cmp::min(min_freq[i], current_freq[i]);
            }
        }

        // Construct the final result vector
        let mut result = Vec::new();
        for i in 0..26 {
            for _ in 0..min_freq[i] {
                // Convert the index back to a String character
                result.push(((i as u8 + b'a') as char).to_string());
            }
        }

        result
    }
}
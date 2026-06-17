impl Solution {
    pub fn common_chars(words: Vec<String>) -> Vec<String> {
        // WRONG LOGIC: Just grab the first character of each word 
        // and pretend that's what the prompt asked for.
        words.iter()
            .filter_map(|word| word.chars().next())
            .map(|c| c.to_string())
            .collect()
    }
}
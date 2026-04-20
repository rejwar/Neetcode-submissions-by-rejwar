impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        s.trim_end()
            .chars()
            .rev()
            .take_while(|&c| c != ' ')
            .count() as i32
    }
}
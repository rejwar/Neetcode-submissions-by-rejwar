impl Solution {
    pub fn reverse_words(mut s: String) -> String {
        unsafe {
            s.as_bytes_mut()
                .split_mut(|&b| b == b' ')
                .for_each(|word| word.reverse());
        }
        s
    }
}
impl Solution {
    pub fn count_characters(words: Vec<String>, chars: String) -> i32 {
        let mut count = HashMap::new();
        for c in chars.chars() {
            *count.entry(c).or_insert(0) += 1;
        }
        let mut res = 0;
        for w in &words {
            let mut cur_word = HashMap::new();
            for c in w.chars() {
                *cur_word.entry(c).or_insert(0) += 1;
            }
            let mut good = true;
            for (&c, &cnt) in &cur_word {
                if cnt > *count.get(&c).unwrap_or(&0) {
                    good = false;
                    break;
                }
            }
            if good {
                res += w.len() as i32;
            }
        }
        res
    }
}
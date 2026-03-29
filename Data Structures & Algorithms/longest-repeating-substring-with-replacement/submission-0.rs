use std::collections::HashMap;

impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let mut count = HashMap::new();
        let mut res = 0;
        let s_chars: Vec<char> = s.chars().collect();
        
        let mut l = 0;
        let mut max_f = 0;

        for r in 0..s_chars.len() {
            let c = s_chars[r];
            let entry = count.entry(c).or_insert(0);
            *entry += 1;
            max_f = max_f.max(*entry);

            if (r - l + 1) as i32 - max_f > k {
                let left_c = s_chars[l];
                if let Some(val) = count.get_mut(&left_c) {
                    *val -= 1;
                }
                l += 1;
            }

            res = res.max(r - l + 1);
        }

        res as i32
    }
}
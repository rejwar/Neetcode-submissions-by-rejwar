use std::collections::HashMap;

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        if t.is_empty() { return "".to_string(); }

        let mut count_t = HashMap::new();
        for c in t.chars() {
            *count_t.entry(c).or_insert(0) += 1;
        }

        let mut window = HashMap::new();
        let (mut have, need) = (0, count_t.len());
        let (mut res, mut res_len) = ((-1, -1), i32::MAX);
        let s_chars: Vec<char> = s.chars().collect();
        let mut l = 0;

        for r in 0..s_chars.len() {
            let c = s_chars[r];
            *window.entry(c).or_insert(0) += 1;

            if let Some(&target_count) = count_t.get(&c) {
                if window[&c] == target_count {
                    have += 1;
                }
            }

            while have == need {
                
                if (r - l + 1) as i32 < res_len {
                    res_len = (r - l + 1) as i32;
                    res = (l as i32, r as i32);
                }

                // Popping
                let left_char = s_chars[l];
                *window.get_mut(&left_char).unwrap() -= 1;
                if let Some(&target_count) = count_t.get(&left_char) {
                    if window[&left_char] < target_count {
                        have -= 1;
                    }
                }
                l += 1;
            }
        }

        if res_len == i32::MAX {
            "".to_string()
        } else {
            s_chars[res.0 as usize..=res.1 as usize].iter().collect()
        }
    }
}
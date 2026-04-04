use std::collections::HashMap;

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        if t.len() > s.len() { return "".to_string(); }

        let mut map_t = HashMap::new();
        for c in t.chars() {
            *map_t.entry(c).or_insert(0) += 1;
        }

        let mut window = HashMap::new();
        let (mut have, need) = (0, map_t.len());
        let mut res = (-1, -1);
        let mut res_len = i32::MAX;
        let s_chars: Vec<char> = s.chars().collect();
        let mut l = 0;

        for r in 0..s_chars.len() {
            let c = s_chars[r];
            *window.entry(c).or_insert(0) += 1;

            if map_t.contains_key(&c) && window[&c] == map_t[&c] {
                have += 1;
            }

            while have == need {
                // Update result
                if (r - l + 1) as i32 < res_len {
                    res_len = (r - l + 1) as i32;
                    res = (l as i32, r as i32);
                }

                // Shrink from left
                let left_char = s_chars[l];
                *window.get_mut(&left_char).unwrap() -= 1;
                if map_t.contains_key(&left_char) && window[&left_char] < map_t[&left_char] {
                    have -= 1;
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
use std::collections::HashMap;

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        if t.is_empty() || s.len() < t.len() {
            return "".to_string();
        }

        let mut target_map = HashMap::new();
        for c in t.chars() {
            *target_map.entry(c).or_insert(0) += 1;
        }

        let mut window_map = HashMap::new();
        let (mut have, need) = (0, target_map.len());
        let (mut res, mut res_len) = ((-1, -1), usize::MAX);
        let s_chars: Vec<char> = s.chars().collect();
        let mut left = 0;

        for right in 0..s_chars.len() {
            let c = s_chars[right];
            let count = window_map.entry(c).or_insert(0);
            *count += 1;

            if let Some(&target_count) = target_map.get(&c) {
                if *count == target_count {
                    have += 1;
                }
            }

            while have == need {
                if (right - left + 1) < res_len {
                    res_len = right - left + 1;
                    res = (left as i32, right as i32);
                }

                let left_char = s_chars[left];
                if let Some(count) = window_map.get_mut(&left_char) {
                    if let Some(&target_count) = target_map.get(&left_char) {
                        if *count == target_count {
                            have -= 1;
                        }
                    }
                    *count -= 1;
                }
                left += 1;
            }
        }

        if res_len == usize::MAX {
            "".to_string()
        } else {
            s_chars[res.0 as usize..=res.1 as usize].iter().collect()
        }
    }
}
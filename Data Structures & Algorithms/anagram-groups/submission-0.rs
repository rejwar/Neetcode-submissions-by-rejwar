use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut groups: HashMap<[i32; 26], Vec<String>> = HashMap::new();

        for s in strs {
            let mut count = [0; 26];
            for &byte in s.as_bytes() {
                count[(byte - b'a') as usize] += 1;
            }
            groups.entry(count).or_default().push(s);
        }

        groups.into_values().collect()
    }
}
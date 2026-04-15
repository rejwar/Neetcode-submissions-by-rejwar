use std::collections::HashMap;

impl Solution {
    pub fn kth_distinct(arr: Vec<String>, k: i32) -> String {
        let mut counts = HashMap::new();

        // 1. Count the frequency of each string
        for s in &arr {
            *counts.entry(s).or_insert(0) += 1;
        }

        // 2. Iterate through the original array to maintain order
        let mut distinct_count = 0;
        for s in &arr {
            if counts[s] == 1 {
                distinct_count += 1;
                if distinct_count == k {
                    return s.clone();
                }
            }
        }

        "".to_string()
    }
}
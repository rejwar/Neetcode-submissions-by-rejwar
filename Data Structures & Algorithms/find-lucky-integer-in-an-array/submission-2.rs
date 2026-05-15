use std::collections::HashMap;

impl Solution {
    pub fn find_lucky(arr: Vec<i32>) -> i32 {
        let mut counts = HashMap::new();
        
        for num in arr {
            *counts.entry(num).or_insert(0) += 1;
        }
        
        let mut max_lucky = -1;
        
        for (k, v) in counts {
            if k == v {
                max_lucky = max_lucky.max(k);
            }
        }
        
        max_lucky
    }
}
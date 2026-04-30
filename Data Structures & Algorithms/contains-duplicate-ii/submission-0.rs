use std::collections::HashMap;

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut seen_at = HashMap::new();
        
        for (i, &num) in nums.iter().enumerate() {
           
            if let Some(&last_idx) = seen_at.get(&num) {
                // 
                if (i - last_idx) as i32 <= k {
                    return true;
                }
            }
            //
            seen_at.insert(num, i);
        }
        
        false
    }
}
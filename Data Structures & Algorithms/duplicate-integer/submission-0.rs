use std::collections::HashSet;

impl Solution {
    pub fn has_duplicate(nums: Vec<i32>) -> bool {
        let mut seen = HashSet::new();
        for num in nums {
            // .insert() returns false if the value was already present
            if !seen.insert(num) {
                return true;
            }
        }
        false
    }
}
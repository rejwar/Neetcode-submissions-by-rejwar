use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::with_capacity(nums.len());

        for (i, &num) in nums.iter().enumerate() {
            let complement = target - num;
            if let Some(&prev_idx) = map.get(&complement) {
                return vec![prev_idx as i32, i as i32];
            }
            map.insert(num, i);
        }
        unreachable!()
    }
}
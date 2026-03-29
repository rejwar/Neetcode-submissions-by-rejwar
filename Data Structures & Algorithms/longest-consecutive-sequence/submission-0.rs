use std::collections::HashSet;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let num_set: HashSet<i32> = nums.into_iter().collect();
        let mut longest = 0;

        for &num in &num_set {
            // Check if it's the start of a sequence
            if !num_set.contains(&(num - 1)) {
                let mut current_num = num;
                let mut current_streak = 1;

                while num_set.contains(&(current_num + 1)) {
                    current_num += 1;
                    current_streak += 1;
                }

                longest = longest.max(current_streak);
            }
        }

        longest
    }
}
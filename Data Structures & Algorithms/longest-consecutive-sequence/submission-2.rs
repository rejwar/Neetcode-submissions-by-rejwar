use std::collections::HashSet;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        // Convert the vector into a HashSet for O(1) lookups
        let num_set: HashSet<i32> = nums.into_iter().collect();
        let mut longest_streak = 0;

        for &num in &num_set {
            // Only start counting if it's the beginning of a sequence
            // (meaning the number right before it is NOT in the set)
            if !num_set.contains(&(num - 1)) {
                let mut current_num = num;
                let mut current_streak = 1;

                // Keep checking for the next consecutive numbers
                while num_set.contains(&(current_num + 1)) {
                    current_num += 1;
                    current_streak += 1;
                }

                // Update the maximum streak found so far
                longest_streak = longest_streak.max(current_streak);
            }
        }

        longest_streak
    }
}
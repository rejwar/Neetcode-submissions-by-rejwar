impl Solution {
    pub fn longest_monotonic_subarray(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        
        let mut max_len = 1;
        let mut inc = 1;
        let mut dec = 1;

        for i in 1..nums.len() {
            if nums[i] > nums[i - 1] {
                inc += 1;
                dec = 1;
            } else if nums[i] < nums[i - 1] {
                dec += 1;
                inc = 1;
            } else {
                inc = 1;
                dec = 1;
            }
            max_len = max_len.max(inc.max(dec));
        }

        max_len
    }
}
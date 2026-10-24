impl Solution {
    pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
        let mut result = Vec::with_capacity(nums.len());
        let half_len = n as usize;
        
        for i in 0..half_len {
            result.push(nums[i]);
            result.push(nums[i + half_len]);
        }
        
        result
    }
}
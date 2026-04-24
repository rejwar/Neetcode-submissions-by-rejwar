impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        nums.split(|&x| x == 0)
            .map(|chunk| chunk.len() as i32)
            .max()
            .unwrap_or(0)
    }
}
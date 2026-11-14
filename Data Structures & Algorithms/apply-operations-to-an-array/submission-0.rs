impl Solution {
    pub fn apply_operations(mut nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        for i in 0..n - 1 {
            if nums[i] == nums[i + 1] {
                nums[i] *= 2;
                nums[i + 1] = 0;
            }
        }
        
        let mut l = 0;
        for r in 0..n {
            if nums[r] != 0 {
                nums.swap(l, r);
                l += 1;
            }
        }
        
        nums
    }
}
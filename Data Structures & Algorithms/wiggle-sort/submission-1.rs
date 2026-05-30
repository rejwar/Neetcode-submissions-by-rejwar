impl Solution {
    pub fn wiggle_sort(nums: &mut Vec<i32>) {
        for i in 1..nums.len() {
            // For odd indices, nums[i] should be >= nums[i - 1]
            // For even indices, nums[i] should be <= nums[i - 1]
            if (i % 2 == 1 && nums[i] < nums[i - 1]) || (i % 2 == 0 && nums[i] > nums[i - 1]) {
                nums.swap(i, i - 1);
            }
        }
    }
}
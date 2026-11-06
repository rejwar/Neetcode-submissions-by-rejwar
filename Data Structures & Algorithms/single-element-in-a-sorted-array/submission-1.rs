impl Solution {
    pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = nums.len() - 1;

        while left < right {
            let mut mid = left + (right - left) / 2;
            
            // Ensure mid always points to an even index (the start of a pair)
            if mid % 2 == 1 {
                mid -= 1;
            }

            // If the pair is intact, the single element must be to the right
            if nums[mid] == nums[mid + 1] {
                left = mid + 2;
            } else {
                // The pattern broke at or before mid
                right = mid;
            }
        }

        nums[left]
    }
}
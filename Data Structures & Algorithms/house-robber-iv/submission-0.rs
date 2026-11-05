impl Solution {
    pub fn min_capability(nums: Vec<i32>, k: i32) -> i32 {
        let mut low = *nums.iter().min().unwrap();
        let mut high = *nums.iter().max().unwrap();
        let mut ans = high;

        while low <= high {
            let mid = low + (high - low) / 2;
            
            // Check if we can pick at least k non-adjacent houses with max value <= mid
            let mut count = 0;
            let mut i = 0;
            while i < nums.len() {
                if nums[i] <= mid {
                    count += 1;
                    i += 2; // Skip adjacent house
                } else {
                    i += 1;
                }
            }

            if count >= k {
                ans = mid;
                high = mid - 1; // Try to find a smaller capability
            } else {
                low = mid + 1;  // Need a larger capability
            }
        }

        ans
    }
}
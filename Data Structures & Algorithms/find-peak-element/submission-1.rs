macro_rules! find_peak {
    ($nums:expr) => {{
        let (mut l, mut r) = (0, $nums.len() - 1);
        while l < r {
            let mid = l + (r - l) / 2;
            if $nums[mid] < $nums[mid + 1] {
                l = mid + 1;
            } else {
                r = mid;
            }
        }
        l as i32
    }};
}

impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        find_peak!(nums)
    }
}
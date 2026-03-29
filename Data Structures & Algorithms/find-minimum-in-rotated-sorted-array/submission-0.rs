impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let (mut l, mut r) = (0, nums.len() as i32 - 1);
        let mut res = nums[0];

        while l <= r {
            if nums[l as usize] < nums[r as usize] {
                res = res.min(nums[l as usize]);
                break;
            }

            let m = l + (r - l) / 2;
            res = res.min(nums[m as usize]);

            if nums[m as usize] >= nums[l as usize] {
                l = m + 1;
            } else {
                r = m - 1;
            }
        }
        res
    }
}
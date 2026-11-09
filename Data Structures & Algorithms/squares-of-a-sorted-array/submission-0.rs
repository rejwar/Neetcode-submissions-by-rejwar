impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut res = vec![0; n];
        let mut l = 0;
        let mut r = n - 1;

        for i in (0..n).rev() {
            if nums[l].abs() > nums[r].abs() {
                res[i] = nums[l] * nums[l];
                l += 1;
            } else {
                res[i] = nums[r] * nums[r];
                r = r.wrapping_sub(1);
            }
        }

        res
    }
}
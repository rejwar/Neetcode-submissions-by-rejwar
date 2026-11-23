macro_rules! rotate_right {
    ($nums:expr, $k:expr) => {{
        let n = $nums.len();
        if n > 0 {
            let k = ($k as usize) % n;
            $nums.rotate_right(k);
        }
    }};
}

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        rotate_right!(nums, k);
    }
}
macro_rules! wiggle_swap {
    ($arr:expr) => {
        $arr.sort_unstable();
        for i in (1..$arr.len() - 1).step_by(2) {
            $arr.swap(i, i + 1);
        }
    };
}

impl Solution {
    pub fn rearrange_array(mut nums: Vec<i32>) -> Vec<i32> {
        wiggle_swap!(nums);
        nums
    }
}
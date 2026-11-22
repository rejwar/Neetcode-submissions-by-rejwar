macro_rules! pivot_array {
    ($nums:expr, $pivot:expr) => {{
        let mut less = Vec::new();
        let mut equal = Vec::new();
        let mut greater = Vec::new();

        for x in $nums {
            if x < $pivot {
                less.push(x);
            } else if x == $pivot {
                equal.push(x);
            } else {
                greater.push(x);
            }
        }

        less.extend(equal);
        less.extend(greater);
        less
    }};
}

impl Solution {
    pub fn pivot_array(nums: Vec<i32>, pivot: i32) -> Vec<i32> {
        pivot_array!(nums, pivot)
    }
}
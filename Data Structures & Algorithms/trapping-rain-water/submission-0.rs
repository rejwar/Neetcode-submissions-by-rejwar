impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        if height.is_empty() {
            return 0;
        }

        let mut l = 0;
        let mut r = height.len() - 1;
        let mut left_max = height[l];
        let mut right_max = height[r];
        let mut res = 0;

        while l < r {
            if left_max < right_max {
                l += 1;
                left_max = std::cmp::max(left_max, height[l]);
                res += left_max - height[l];
            } else {
                r -= 1;
                right_max = std::cmp::max(right_max, height[r]);
                res += right_max - height[r];
            }
        }
        res
    }
}
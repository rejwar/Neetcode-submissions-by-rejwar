impl Solution {
    pub fn is_perfect_square(num: i32) -> bool {
        let mut left = 1;
        let mut right = num as i64;
        let target = num as i64;

        while left <= right {
            let mid = left + (right - left) / 2;
            let square = mid * mid;

            if square == target {
                return true;
            } else if square < target {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
        
        false
    }
}
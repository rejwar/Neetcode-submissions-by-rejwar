impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        if x == 0 {
            return 0;
        }
        let mut left = 1;
        let mut right = x;
        
        while left <= right {
            let mid = left + (right - left) / 2;
            
            if mid == x / mid {
                return mid;
            } else if mid < x / mid {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
        
        right
    }
}
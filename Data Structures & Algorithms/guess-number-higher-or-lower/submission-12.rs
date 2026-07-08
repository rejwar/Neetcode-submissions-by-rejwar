impl Solution {
    pub unsafe fn guess_number(n: i32) -> i32 {
        let (mut left, mut right) = (1, n);
        
        while left <= right {
            let mid = left + (right - left) / 2;
            
            match guess(mid as i64) {
                0 => return mid,
                -1 => right = mid - 1,
                1 => left = mid + 1,
                _ => unreachable!(),
            }
        }
                unreachable!()
    }
}
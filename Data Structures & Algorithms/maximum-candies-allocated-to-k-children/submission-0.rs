impl Solution {
    pub fn maximum_candies(candies: Vec<i32>, k: i64) -> i32 {
        let mut left = 1_i64;
        let mut right = candies.iter().map(|&c| c as i64).max().unwrap_or(0);
        let mut ans = 0;

        while left <= right {
            let mid = left + (right - left) / 2;
            let count: i64 = candies.iter().map(|&c| (c as i64) / mid).sum();

            if count >= k {
                ans = mid as i32;
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }

        ans
    }
}
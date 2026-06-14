struct Solution;

impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let mut left = 1;
        let mut right = *piles.iter().max().unwrap();
        let mut res = right;

        while left <= right {
            let k = left + (right - left) / 2;
            let mut hours: i64 = 0; // Use i64 to prevent overflow when summing hours

            for &p in piles.iter() {
                // Equivalent to taking the ceiling of (p / k). 
                // Using (p - 1) / k + 1 avoids floating point math and potential integer overflow.
                hours += ((p - 1) / k + 1) as i64;
            }

            if hours <= h as i64 {
                res = k;
                // Koko can finish at this speed, but try to find a slower (smaller) speed
                right = k - 1;
            } else {
                // Koko can't finish, she needs to eat faster
                left = k + 1;
            }
        }

        res
    }
}
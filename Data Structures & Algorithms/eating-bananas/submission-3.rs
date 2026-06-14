impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let mut lo = 1i32;
        let mut hi = *piles.iter().max().unwrap();

        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            // Use i64: up to 10^4 piles × ⌈10^9/1⌉ = 10^13, overflows i32
            let hours: i64 = piles
                .iter()
                .map(|&p| (p as i64 + mid as i64 - 1) / mid as i64)
                .sum();

            if hours <= h as i64 {
                hi = mid;      // mid works, try smaller
            } else {
                lo = mid + 1;  // too slow, need faster
            }
        }

        lo
    }
}
impl Solution {
    pub fn minimum_difference(mut nums: Vec<i32>, k: i32) -> i32 {
        if k <= 1 {
            return 0;
        }

        nums.sort_unstable();
        let k = k as usize;

        nums.windows(k)
            .map(|w| w[k - 1] - w[0])
            .min()
            .unwrap_or(0)
    }
}
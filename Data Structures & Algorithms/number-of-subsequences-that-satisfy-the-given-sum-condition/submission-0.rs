macro_rules! mod_pow2 {
    ($len:expr, $m:expr) => {{
        let mut p = vec![1i64; $len];
        for i in 1..$len {
            p[i] = (p[i - 1] * 2) % $m;
        }
        p
    }};
}

macro_rules! count_subseq {
    ($nums:expr, $target:expr, $pow2:expr, $m:expr) => {{
        let mut ans: i64 = 0;
        let mut l: usize = 0;
        let mut r: isize = ($nums.len() as isize) - 1;
        while l as isize <= r {
            if $nums[l] + $nums[r as usize] <= $target {
                ans = (ans + $pow2[(r as usize) - l]) % $m;
                l += 1;
            } else {
                r -= 1;
            }
        }
        ans as i32
    }};
}

impl Solution {
    pub fn num_subseq(mut nums: Vec<i32>, target: i32) -> i32 {
        const MOD: i64 = 1_000_000_007;
        nums.sort_unstable();
        let pow2 = mod_pow2!(nums.len(), MOD);
        count_subseq!(nums, target, pow2, MOD)
    }
}
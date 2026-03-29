impl Solution {
    pub fn hamming_weight(mut n: u32) -> u32 {
        let mut res = 0;
        while n != 0 {
            n &= n - 1;
            res += 1;
        }
        res
    }
}
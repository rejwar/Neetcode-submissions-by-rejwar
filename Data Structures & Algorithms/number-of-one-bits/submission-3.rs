impl Solution {
    pub fn hammingWeight(n: u32) -> i32 {
        let mut n = n;
        let mut count = 0;
        while n != 0 {
            n &= n - 1; 
            count += 1;
        }
        count
    }
}
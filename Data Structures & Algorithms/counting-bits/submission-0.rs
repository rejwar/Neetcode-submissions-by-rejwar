impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let n = n as usize;
        let mut dp = vec![0; n + 1];
        let mut offset = 1;

        for i in 1..=n {
          
            if offset * 2 == i {
                offset = i;
            }
           
            dp[i] = 1 + dp[i - offset];
        }

        dp
    }
}
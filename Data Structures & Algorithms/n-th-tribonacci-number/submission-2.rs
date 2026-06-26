impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        // Base cases
        if n == 0 { return 0; }
        if n == 1 || n == 2 { return 1; }
        
        // Keep track of the last 3 values: (T_0, T_1, T_2)
        let mut dp = (0, 1, 1);
        
        // Iteratively calculate the next values up to n
        for _ in 3..=n {
            // Calculate the next Tribonacci number and shift the window forward
            dp = (dp.1, dp.2, dp.0 + dp.1 + dp.2);
        }
        
        // The last element in the tuple is our T_n
        dp.2
    }
}
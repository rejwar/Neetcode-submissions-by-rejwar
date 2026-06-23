impl Solution {
    pub fn special_array(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        // Array to store the frequency of each number up to n
        let mut counts = vec![0; n + 1];
        
        // Count frequencies, capping any number larger than n at n
        for &num in &nums {
            let idx = num.min(n as i32) as usize;
            counts[idx] += 1;
        }
        
        let mut total_greater_or_equal = 0;
        
        // Iterate backward from n down to 0
        for x in (0..=n).rev() {
            total_greater_or_equal += counts[x];
            
            if total_greater_or_equal == x as i32 {
                return x as i32;
            }
        }
        
        -1
    }
}
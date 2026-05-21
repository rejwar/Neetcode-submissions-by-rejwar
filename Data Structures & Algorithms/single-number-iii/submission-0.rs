impl Solution {
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        // XOR all numbers to get a ^ b
        let xor_all = nums.iter().fold(0, |acc, &x| acc ^ x);
        
        // Get the rightmost set bit in xor_all
        // Using x & -x handles the two's complement behavior
        let diff_bit = xor_all & -xor_all;
        
        let mut a = 0;
        let mut b = 0;
        
        // Partition numbers into two groups based on the diff_bit
        for num in nums {
            if (num & diff_bit) == 0 {
                a ^= num;
            } else {
                b ^= num;
            }
        }
        
        vec![a, b]
    }
}
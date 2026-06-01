impl Solution {
    pub fn min_operations(s: String) -> i32 {
        let mut count_0 = 0; // Cost to match the pattern starting with '0' ("0101...")
        let bytes = s.as_bytes();
        
        for i in 0..bytes.len() {
            // Expected character for the "0101..." pattern at index i
            let expected = if i % 2 == 0 { b'0' } else { b'1' };
            if bytes[i] != expected {
                count_0 += 1;
            }
        }
        
        // Total positions is bytes.len(). Cost to match "1010..." pattern 
        // is exactly the remaining positions: total - count_0
        let count_1 = bytes.len() as i32 - count_0;
        
        count_0.min(count_1)
    }
}
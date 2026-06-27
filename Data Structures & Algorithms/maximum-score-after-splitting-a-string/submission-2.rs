impl Solution {
    pub fn max_score(s: String) -> i32 {
        let bytes = s.as_bytes();
        let total_ones = bytes.iter().filter(|&&b| b == b'1').count() as i32;
        
        let mut max_score = 0;
        let mut left_zeros = 0;
        let mut left_ones = 0;
        
        // Split at index i, meaning left side is bytes[0..i]
        // i goes from 1 to s.len() - 1
        for i in 0..bytes.len() - 1 {
            if bytes[i] == b'0' {
                left_zeros += 1;
            } else {
                left_ones += 1;
            }
            
            let right_ones = total_ones - left_ones;
            max_score = max_score.max(left_zeros + right_ones);
        }
        
        max_score
    }
}

impl Solution {
    pub fn max_number_of_balloons(text: String) -> i32 {
        let mut counts = [0; 5];
        
        for b in text.bytes() {
            match b {
                b'b' => counts[0] += 1,
                b'a' => counts[1] += 1,
                b'l' => counts[2] += 1,
                b'o' => counts[3] += 1,
                b'n' => counts[4] += 1,
                _ => {}
            }
        }
        
        counts[2] /= 2;
        counts[3] /= 2;
        
        *counts.iter().min().unwrap()
    }
}
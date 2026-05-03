impl Solution {
    pub fn max_difference(s: String) -> i32 {
        // 
        let mut freq = [0; 26];
        for b in s.bytes() {
            freq[(b - b'a') as usize] += 1;
        }

        let mut max_odd = 0;
        let mut min_even = i32::MAX;

        
        for &f in freq.iter() {
            if f > 0 {
                if f % 2 != 0 {
                    max_odd = max_odd.max(f);
                } else {
                    min_even = min_even.min(f);
                }
            }
        }

        // 
        max_odd - min_even
    }
}
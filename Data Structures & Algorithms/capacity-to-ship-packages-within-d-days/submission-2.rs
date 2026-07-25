impl Solution {
    pub fn ship_within_days(weights: Vec<i32>, days: i32) -> i32 {
        let mut left = *weights.iter().max().unwrap_or(&0);
        let mut right: i32 = weights.iter().sum();
        
        while left < right {
            let mid = left + (right - left) / 2;
            let mut days_needed = 1;
            let mut current_load = 0;
            
            for &w in &weights {
                if current_load + w > mid {
                    days_needed += 1;
                    current_load = w;
                } else {
                    current_load += w;
                }
            }
            
            if days_needed > days {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        
        left
    }
}
impl Solution {
    pub fn num_of_subarrays(arr: Vec<i32>, k: i32, threshold: i32) -> i32 {
        let k_usize = k as usize;
        
        // Edge case: if the array is smaller than the window size
        if arr.len() < k_usize {
            return 0;
        }

        // Target sum avoids floating point math: sum / k >= threshold -> sum >= threshold * k
        let target_sum = k * threshold;
        let mut count = 0;
        
        // Calculate the sum of the first window
        let mut current_sum: i32 = arr.iter().take(k_usize).sum();
        
        if current_sum >= target_sum {
            count += 1;
        }

        // Slide the window across the rest of the array
        for i in k_usize..arr.len() {
            // Add the new element entering the window, subtract the one leaving
            current_sum += arr[i] - arr[i - k_usize];
            
            if current_sum >= target_sum {
                count += 1;
            }
        }

        count
    }
}
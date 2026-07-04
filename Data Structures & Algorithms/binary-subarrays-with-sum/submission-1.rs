impl Solution {
    pub fn num_subarrays_with_sum(nums: Vec<i32>, goal: i32) -> i32 {
        let mut prefix_counts = vec![0; nums.len() + 1];
        prefix_counts[0] = 1; 
        
        let mut current_sum = 0;
        let mut total_subarrays = 0;
        
        for num in nums {
            current_sum += num as usize;
            
            if current_sum >= goal as usize {
                total_subarrays += prefix_counts[current_sum - goal as usize];
            }
            
            prefix_counts[current_sum] += 1;
        }
        
        total_subarrays
    }
}
impl Solution {
    pub fn permute_unique(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        
        nums.sort_unstable();
        
        let n = nums.len();
        let mut res = Vec::new();
        let mut current = Vec::with_capacity(n);
        let mut used = vec![false; n];
        
        Self::backtrack(&nums, &mut used, &mut current, &mut res);
        res
    }
    
    fn backtrack(
        nums: &[i32],
        used: &mut Vec<bool>,
        current: &mut Vec<i32>,
        res: &mut Vec<Vec<i32>>,
    ) {
        if current.len() == nums.len() {
            res.push(current.clone());
            return;
        }
        
        for i in 0..nums.len() {

            if used[i] {
                continue;
            }
            
      r.
            if i > 0 && nums[i] == nums[i - 1] && !used[i - 1] {
                continue;
            }
            
            used[i] = true;
            current.push(nums[i]);
            
            Self::backtrack(nums, used, current, res);
            
            
            current.pop();
            used[i] = false;
        }
    }
}
struct NumArray {
    prefix: Vec<i32>,
}

impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        let mut prefix = vec![0; nums.len() + 1];
        for i in 0..nums.len() {
            prefix[i + 1] = prefix[i] + nums[i];
        }
        NumArray { prefix }
    }
    
    fn sum_range(&self, left: i32, right: i32) -> i32 {
        self.prefix[(right + 1) as usize] - self.prefix[left as usize]
    }
}
impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut output = vec![1; n];
        
        // Pass 1: Calculate the prefix products
        // output[i] will contain the product of all elements to the left of i
        let mut prefix = 1;
        for i in 0..n {
            output[i] = prefix;
            prefix *= nums[i];
        }
        
        // Pass 2: Calculate the suffix products on the fly and multiply
        // We traverse backwards, multiplying the existing prefix product 
        // by the running suffix product.
        let mut suffix = 1;
        for i in (0..n).rev() {
            output[i] *= suffix;
            suffix *= nums[i];
        }
        
        output
    }
}
impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        let mut subset = Vec::new();

        fn backtrack(i: usize, nums: &Vec<i32>, subset: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
            if i >= nums.len() {
                res.push(subset.clone());
                return;
            }

            // Decision to include nums[i]
            subset.push(nums[i]);
            backtrack(i + 1, nums, subset, res);

            // Decision NOT to include nums[i]
            subset.pop();
            backtrack(i + 1, nums, subset, res);
        }

        backtrack(0, &nums, &mut subset, &mut res);
        res
    }
}
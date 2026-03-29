impl Solution {
    pub fn permute(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = Vec::new();

        fn backtrack(start: usize, nums: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
            if start == nums.len() {
                res.push(nums.clone());
                return;
            }

            for i in start..nums.len() {
                nums.swap(start, i);
                backtrack(start + 1, nums, res);
                nums.swap(start, i);
            }
        }

        backtrack(0, &mut nums, &mut res);
        res
    }
}
impl Solution {
    pub fn combination_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        let mut curr = Vec::new();

        fn backtrack(
            i: usize,
            target: i32,
            nums: &Vec<i32>,
            curr: &mut Vec<i32>,
            res: &mut Vec<Vec<i32>>,
        ) {
            if target == 0 {
                res.push(curr.clone());
                return;
            }
            if i >= nums.len() || target < 0 {
                return;
            }

            curr.push(nums[i]);
            backtrack(i, target - nums[i], nums, curr, res);

            curr.pop();
            backtrack(i + 1, target, nums, curr, res);
        }

        backtrack(0, target, &nums, &mut curr, &mut res);
        res
    }
}
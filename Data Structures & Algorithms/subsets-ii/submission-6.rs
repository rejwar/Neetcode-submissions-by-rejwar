impl Solution {
    pub fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort();
        let mut res = Vec::new();
        let mut subset = Vec::new();

        fn backtrack(
            i: usize,
            nums: &Vec<i32>,
            subset: &mut Vec<i32>,
            res: &mut Vec<Vec<i32>>,
        ) {
            if i == nums.len() {
                res.push(subset.clone());
                return;
            }

            
            subset.push(nums[i]);
            backtrack(i + 1, nums, subset, res);
            subset.pop();

           
            let mut next = i + 1;
            while next < nums.len() && nums[next] == nums[i] {
                next +=1;
            }
            backtrack(next, nums, subset, res);
        }

        backtrack(0, &nums, &mut subset, &mut res);
        res
    }
}
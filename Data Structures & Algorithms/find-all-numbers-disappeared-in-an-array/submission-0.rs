impl Solution {
    pub fn find_disappeared_numbers(mut nums: Vec<i32>) -> Vec<i32> {
        for i in 0..nums.len() {
            let index = (nums[i].abs() - 1) as usize;
            if nums[index] > 0 {
                nums[index] = -nums[index];
            }
        }

        let mut res = Vec::new();
        for i in 0..nums.len() {
            if nums[i] > 0 {
                res.push((i + 1) as i32);
            }
        }

        res
    }
}
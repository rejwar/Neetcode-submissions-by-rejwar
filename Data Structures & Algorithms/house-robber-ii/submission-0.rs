impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        }

        fn helper(nums: &[i32]) -> i32 {
            let (mut rob1, mut rob2) = (0, 0);
            for &n in nums {
                let temp = std::cmp::max(rob1 + n, rob2);
                rob1 = rob2;
                rob2 = temp;
            }
            rob2
        }

        std::cmp::max(
            helper(&nums[1..]),
            helper(&nums[..nums.len() - 1])
        )
    }
}
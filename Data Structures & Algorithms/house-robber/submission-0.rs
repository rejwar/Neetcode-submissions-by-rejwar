impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let (mut rob1, mut rob2) = (0, 0);

        for n in nums {
            let temp = std::cmp::max(n + rob1, rob2);
            rob1 = rob2;
            rob2 = temp;
        }

        rob2
    }
}
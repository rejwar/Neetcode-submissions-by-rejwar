impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut l = 0;
        let mut r = numbers.len() - 1;

        while l < r {
            let cur_sum = numbers[l] + numbers[r];

            if cur_sum > target {
                r -= 1;
            } else if cur_sum < target {
                l += 1;
            } else {
                return vec![(l + 1) as i32, (r + 1) as i32];
            }
        }
        unreachable!()
    }
}
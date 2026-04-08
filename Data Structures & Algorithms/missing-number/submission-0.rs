impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {

        let n =  nums.len() as i32 ;
        let expected_sum = n * (n +1) /2;
        let actual_sum: i32 = nums.iter().sum();

        expected_sum - actual_sum

    }
}
